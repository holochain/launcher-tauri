import { AdminWebsocket, AppAgentWebsocket } from "@holochain/client";
import { execSync } from "child_process";
import yaml from "js-yaml";
import fs from "fs";
import crypto from "crypto";
import AdmZip from "adm-zip";

const INFO_LOGO_PATH = `${process.cwd()}/public/img/info_icon.svg`;
const TESTING_APPS_PATH = `${process.cwd()}/testing-apps`;

function getAppsPaths() {
  return fs
    .readdirSync(TESTING_APPS_PATH)
    .filter((p) => p.endsWith(".webhapp"));
}

async function publishApps() {
  const adminWs = await AdminWebsocket.connect(
    new URL(`ws://127.0.0.1:${process.env.ADMIN_PORT}`),
    100000
  );

  const apps = await adminWs.listApps({});

  const devhubAppId = apps.find((app) =>
    app.installed_app_id.includes("DevHub")
  ).installed_app_id;

  const appstoreAppId = apps.find((app) =>
    app.installed_app_id.includes("AppStore")
  ).installed_app_id;

  const appPorts = await adminWs.listAppInterfaces();
  const devhubClient = await AppAgentWebsocket.connect(
    `ws://127.0.0.1:${appPorts[0]}`,
    devhubAppId,
    100000
  );
  const appstoreClient = await AppAgentWebsocket.connect(
    `ws://127.0.0.1:${appPorts[0]}`,
    appstoreAppId,
    100000
  );
  const devhubCells = await devhubClient.appInfo();
  for (const [role_name, [cell]] of Object.entries(devhubCells.cell_info)) {
    await adminWs.authorizeSigningCredentials(cell["provisioned"].cell_id, {
      All: null,
    });
  }
  const appstoreCells = await appstoreClient.appInfo();
  for (const [role_name, [cell]] of Object.entries(appstoreCells.cell_info)) {
    await adminWs.authorizeSigningCredentials(cell["provisioned"].cell_id, {
      All: null,
    });
  }

  const publisher = await appstoreClient.callZome({
    role_name: "appstore",
    zome_name: "appstore_api",
    fn_name: "create_publisher",
    payload: {
      name: "holo",
      location: {
        country: "es",
        region: "Catalunya",
        city: "Barcelona",
      },
      website: { url: "https://lightningrodlabs.org", context: "website" },
      icon: crypto.randomBytes(1_000),
      email: "some@email.org",
      editors: [],
    },
  });

  const allAppsOutput = await appstoreClient.callZome({
    role_name: "appstore",
    zome_name: "appstore_api",
    fn_name: "get_all_apps",
    payload: null,
  });
  const appsPaths = getAppsPaths();
  if (appsPaths.length === 0) {
    throw new Error("No apps found in testing-apps directory");
  }
  for (const path of appsPaths) {
    const appName = path.split(".")[0];
    if (allAppsOutput.payload.find((app) => app.content.title === appName)) {
      console.log(`App ${appName} already published`);
      continue;
    }

    console.log(`Publishing ${appName}...`);
    unpackWebHapp(appName);

    const dnaEntities = [];

    const dnas = fs
      .readdirSync(`${TESTING_APPS_PATH}/${appName}/happ/dnas`)
      .filter((name) => !name.endsWith("dna"));

    for (const dna of dnas) {
      const dnaManifest = yaml.load(
        fs.readFileSync(
          `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna}/dna.yaml`
        )
      );

      const integrityZomeEntities = [];
      const coordinatorZomeEntities = [];
      for (const izome of dnaManifest.integrity.zomes) {
        const izomeEntity = await devhubClient.callZome({
          role_name: "dnarepo",
          zome_name: "dna_library",
          fn_name: "create_zome",
          payload: {
            name: izome.name,
            zome_type: 0,
            display_name: izome.name,
            description: "",
          },
        });
        const zome_bytes = fs.readFileSync(
          `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna}/zomes/integrity/${izome.name}.wasm`
        );
        const izomeVersionEntity = await devhubClient.callZome({
          role_name: "dnarepo",
          zome_name: "dna_library",
          fn_name: "create_zome_version",
          payload: {
            for_zome: izomeEntity.payload.id,
            version: "0.1",
            ordering: 1,
            zome_bytes,
            hdk_version: "v0.2.1",
          },
        });

        integrityZomeEntities.push([izomeEntity, izomeVersionEntity]);
      }
      for (const czome of dnaManifest.coordinator.zomes) {
        const czomeEntity = await devhubClient.callZome({
          role_name: "dnarepo",
          zome_name: "dna_library",
          fn_name: "create_zome",
          payload: {
            name: czome.name,
            zome_type: 1,
            display_name: czome.name,
            description: "",
          },
        });
        const zome_bytes = fs.readFileSync(
          `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna}/zomes/coordinator/${czome.name}.wasm`
        );
        const czomeVersionEntity = await devhubClient.callZome({
          role_name: "dnarepo",
          zome_name: "dna_library",
          fn_name: "create_zome_version",
          payload: {
            for_zome: czomeEntity.payload.id,
            version: "0.1",
            ordering: 1,
            zome_bytes,
            hdk_version: "v0.2.1",
          },
        });
        coordinatorZomeEntities.push([czomeEntity, czomeVersionEntity]);
      }
      const dnaEntity = await devhubClient.callZome({
        role_name: "dnarepo",
        zome_name: "dna_library",
        fn_name: "create_dna",
        payload: {
          name: dna,
          display_name: dna,
          description: "",
        },
      });
      const dnaVersionEntity = await devhubClient.callZome({
        role_name: "dnarepo",
        zome_name: "dna_library",
        fn_name: "create_dna_version",
        payload: {
          for_dna: dnaEntity.payload.id,
          version: "0.1",
          ordering: 1,
          hdk_version: "v0.2.1",
          integrity_zomes: integrityZomeEntities.map(([ze, zve]) => ({
            name: ze.payload.content.name,
            zome: zve.payload.content.for_zome,
            version: zve.payload.id,
            resource: zve.payload.content.mere_memory_addr,
            resource_hash: zve.payload.content.mere_memory_hash,
          })),
          zomes: coordinatorZomeEntities.map(([ze, zve]) => ({
            name: ze.payload.content.name,
            zome: zve.payload.content.for_zome,
            version: zve.payload.id,
            resource: zve.payload.content.mere_memory_addr,
            resource_hash: zve.payload.content.mere_memory_hash,
            dependencies: dnaManifest.coordinator.zomes
              .find((z) => z.name === ze.payload.content.name)
              .dependencies.map((d) => d.name),
          })),
          origin_time: "2022-02-11T23:05:19.470323Z",
        },
      });

      dnaEntities.push([dnaEntity, dnaVersionEntity]);
    }

    const happManifest = yaml.load(
      fs.readFileSync(`${TESTING_APPS_PATH}/${appName}/happ/happ.yaml`)
    );

    const appEntity = await devhubClient.callZome({
      role_name: "happs",
      zome_name: "happ_library",
      fn_name: "create_happ",
      payload: {
        title: appName,
        subtitle: happManifest.description || "",
        description: "",
        tags: ["we-app"],
      },
    });

    const file_bytes = fs.readFileSync(
      `${TESTING_APPS_PATH}/${appName}/ui.zip`
    );

    const fileEntity = await devhubClient.callZome({
      role_name: "web_assets",
      zome_name: "web_assets",
      fn_name: "create_file",
      payload: {
        file_bytes,
      },
    });
    const guiEntity = await devhubClient.callZome({
      role_name: "happs",
      zome_name: "happ_library",
      fn_name: "create_gui",
      payload: {
        name: "UI",
        description: "",
      },
    });
    const guiVersionEntity = await devhubClient.callZome({
      role_name: "happs",
      zome_name: "happ_library",
      fn_name: "create_gui_release",
      payload: {
        version: "0.1",
        changelog: "",
        for_gui: guiEntity.payload.id,
        for_happ_releases: [],
        web_asset_id: fileEntity.payload.id,
      },
    });
    const appVersionEntity = await devhubClient.callZome({
      role_name: "happs",
      zome_name: "happ_library",
      fn_name: "create_happ_release",
      payload: {
        version: "0.1",
        description: "",
        for_happ: appEntity.payload.id,
        ordering: 1,
        manifest: happManifest,
        official_gui: guiVersionEntity.payload.id,
        hdk_version: "v0.1.0",
        dnas: dnaEntities.map(([de, dve]) => ({
          role_name: de.payload.content.name,
          dna: de.payload.id,
          version: dve.payload.id,
          wasm_hash: dve.payload.content.wasm_hash,
        })),
      },
    });

    const happlibraryDnaHash =
      devhubCells.cell_info["happs"][0]["provisioned"].cell_id[0];

    // check whether there is an icon in the ui.zip, otherwise take a default icon
    let iconBytes = fs.readFileSync(INFO_LOGO_PATH); // default icon
    let icon_mime_type = "image/svg+xml";

    let zip = new AdmZip(file_bytes);
    const zipEntries = zip.getEntries();
    zipEntries.forEach((entry) => {
      if (entry.entryName === "icon.png") {
        iconBytes = entry.getData();
        icon_mime_type = "image/png";
      }
    });

    await appstoreClient.callZome({
      role_name: "appstore",
      zome_name: "appstore_api",
      fn_name: "create_app",
      payload: {
        title: appName,
        subtitle: happManifest.description || "",
        description: "",
        icon: iconBytes,
        publisher: publisher.payload.id,
        devhub_address: {
          dna: happlibraryDnaHash,
          happ: appEntity.payload.id,
          gui: guiVersionEntity.payload.id,
        },
        editors: [],
        metadata: {
          icon_mime_type,
        },
      },
    });

    console.log("Published app: ", appName);
  }
}

// Output structure:
// testing-apps/gather
// ├── gather.happ
// ├── happ
// │   ├── dnas
// │   │   ├── gather
// │   │   │   ├── dna.yaml
// │   │   │   └── zomes
// │   │   │       ├── coordinator
// │   │   │       │   ├── file_storage.wasm
// │   │   │       │   ├── gather.wasm
// │   │   │       │   └── profiles.wasm
// │   │   │       └── integrity
// │   │   │           ├── file_storage_integrity.wasm
// │   │   │           ├── gather_integrity.wasm
// │   │   │           └── profiles_integrity.wasm
// │   │   └── gather.dna
// │   └── happ.yaml
// ├── ui.zip
// └── web-happ.yaml
function unpackWebHapp(appName) {
  if (fs.existsSync(`${TESTING_APPS_PATH}/${appName}`)) return;
  execSync(
    `hc web-app unpack ${TESTING_APPS_PATH}/${appName}.webhapp -o ${TESTING_APPS_PATH}/${appName}`
  );
  const webhapp = yaml.load(
    fs.readFileSync(`${TESTING_APPS_PATH}/${appName}/web-happ.yaml`)
  );

  fs.renameSync(
    `${TESTING_APPS_PATH}/${appName}/${webhapp.happ_manifest.bundled}`,
    `${TESTING_APPS_PATH}/${appName}/${appName}.happ`
  );
  fs.renameSync(
    `${TESTING_APPS_PATH}/${appName}/${webhapp.ui.bundled}`,
    `${TESTING_APPS_PATH}/${appName}/ui.zip`
  );

  execSync(
    `hc app unpack ${TESTING_APPS_PATH}/${appName}/${appName}.happ -o ${TESTING_APPS_PATH}/${appName}/happ`
  );
  const happManifest = yaml.load(
    fs.readFileSync(`${TESTING_APPS_PATH}/${appName}/happ/happ.yaml`)
  );

  fs.mkdirSync(`${TESTING_APPS_PATH}/${appName}/happ/dnas`);
  for (const dna of happManifest.roles) {
    fs.renameSync(
      `${TESTING_APPS_PATH}/${appName}/happ/${dna.dna.bundled}`,
      `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}.dna`
    );
    execSync(
      `hc dna unpack ${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}.dna -o ${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}`
    );

    const dnaManifest = yaml.load(
      fs.readFileSync(
        `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}/dna.yaml`
      )
    );
    fs.mkdirSync(`${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}/zomes`);
    fs.mkdirSync(
      `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}/zomes/integrity`
    );
    fs.mkdirSync(
      `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}/zomes/coordinator`
    );

    for (const zome of dnaManifest.integrity.zomes) {
      fs.renameSync(
        `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}/${zome.bundled}`,
        `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}/zomes/integrity/${zome.name}.wasm`
      );
    }

    for (const zome of dnaManifest.coordinator.zomes) {
      fs.renameSync(
        `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}/${zome.bundled}`,
        `${TESTING_APPS_PATH}/${appName}/happ/dnas/${dna.name}/zomes/coordinator/${zome.name}.wasm`
      );
    }
  }
}

async function publishAppsRetry() {
  try {
    await publishApps();
  } catch (e) {
    if (e.toString().includes("could not connect to holochain conductor")) {
      console.log(e.toString());
      console.log(
        "Couldn't publish apps yet because the conductor is still setting up, have you entered your password and enabled the developer mode? Retrying again in a few seconds..."
      );
    } else if (
      e.toString().includes("crypto.getRandomValues is not a function")
    ) {
      console.log("Failed to publish apps: Error: ", e);
      console.log(
        "\n\nMake sure to use a recent enough version of node (>18). Check your node version with 'node --version'."
      );
    } else if (
      e
        .toString()
        .includes(
          "TypeError: Cannot read properties of undefined (reading 'installed_app_id')"
        )
    ) {
      console.log("Failed to publish apps: Error: ", e);
      console.log("\n\nYou probably haven't installed the DevHub yet.");
    } else if (e.toString().includes("syntax error near unexpected token")) {
      console.log(
        "Check the name of the webhapp file. There might be unexpected syntax in the name."
      );
    } else if (e.toString().includes("testing-apps")) {
      console.log(
        "You probably haven't add webhapp`s file to testing-apps directory"
      );
    } else if (e.toString().includes("hc: command not found")) {
      console.log(
        "Remember to run this script from within a nix-shell. So the hc command will be available."
      );
    } else {
      console.log("Failed to publish apps. Error: ", e);
    }
    setTimeout(async () => {
      await publishAppsRetry();
    }, 15000);
  }
}
publishAppsRetry();

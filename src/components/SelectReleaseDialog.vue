<template>
  <HCDialog ref="dialog" @closing="$emit('closing-dialog')" closeOnSideClick>

    <div
      class="column"
      style="align-items: center; margin: 10px 15px; padding: 10px 20px"
    >

      <div class="column" style="width: 100%; align-items: flex-start; margin: 12px 0 40px 0;">

        <div class="row" style="align-items: center; margin-bottom: 20px;">
          <!-- if icon provided -->
          <img
            v-if="imgSrc"
            :src="imgSrc"
            alt="app icon"
            style="
              width: 120px;
              height: 120px;
              border-radius: 12px;
              object-fit: cover;
            "
          />
          <!-- if no icon provided -->
          <div
            v-else
            class="column center-content"
            style="
              width: 120px;
              height: 120px;
              border-radius: 12px;
              background: #372ba5;
              margin: 15px;
            "
          >
            <div style="color: white; font-size: 55px; font-weight: 600">
              {{ app.title.slice(0, 2) }}
            </div>
          </div>
          <div
            style="
              font-size: 30px;
              font-weight: 600;
              margin: 0 0 0 30px;
              line-height: 115%;
              word-break: normal;
            "
            :title="app.title"
          >
            {{ app.title }}
          </div>
        </div>

        <div style="margin-left: 10px;"><span style="font-weight: 600;">published by: </span>{{ publisher? `${publisher.name}, ${publisher.location.country}` : "unknown" }}</div>
        <div style="margin-left: 10px;"><span style="font-weight: 600;">first published: </span>{{ (new Date(app.published_at)).toLocaleDateString(locale) }}</div>

        <div style="font-weight: 600; font-size: 20px; margin: 20px 0 10px 0">
          Description:
        </div>

        <div style="max-height: 200px; min-width: 610px; overflow-y: auto; background: #f6f6fa; padding: 5px 10px; border-radius: 10px;">
          {{ app.description }}
        </div>

        <div style="font-weight: 600; font-size: 20px; margin: 30px 0 10px 0">
          Available Releases:
        </div>

        <div v-if="releaseDatas && releaseDatas.length > 0">
          <div class="column card">
            <div style="text-align: right; font-weight: 600">latest Release</div>

            <div class="row" style="align-items: flex-end;">
              <div class="column">
                <div><span style="font-weight: 600;">hApp version:</span> {{ releaseDatas[0].happRelease.content.version }}</div>
                <div><span style="font-weight: 600;">UI version: </span>{{ releaseDatas[0].guiRelease? releaseDatas[0].guiRelease.content.version : "no official UI" }}</div>
                <!-- GUI version as well here? Needs to be fetched independently from a DevHub host -->
              </div>
              <span style="display: flex; flex: 1;"></span>
              <HCButton @click="() => {
                $emit('release-selected', releaseDatas ? releaseDatas[0] : undefined);
                close();
              }">Install</HCButton>
            </div>
          </div>


          <div class="row" style="margin: 20px 0 10px 10px;">
            <div
              @click="showAdvanced = !showAdvanced"
              @keydown.enter="showAdvanced = !showAdvanced"
              class="row advanced-button"
              style="align-items: center;"
              tabindex="0"
            >
              <div
                style="
                  text-align: center;
                  font-size: 24px;
                  width: 20px;
                  cursor: pointer;
                "
              >
                {{ showAdvanced ? "-" : "+" }}
              </div>
              <div style="font-size: 18px; margin-left: 10px">show older releases</div>
            </div>
            <span style="display: flex; flex: 1;"></span>
          </div>

          <div v-show="showAdvanced" class="column" style="align-items: center; width: 100%;">
            <div v-if="releaseDatas.length < 2" style="text-align: center;">
              No older releases available for this app.
            </div>
            <div v-else>
              <div
                v-for="(releaseData) of releaseDatas.slice(1)"
                :key="releaseData.happRelease.content.version"
                class="row card"
                style="align-items: flex-end; padding-top: 15px;"
              >
                <div class="column">
                  <div><span style="font-weight: 600;">hApp version:</span> {{ releaseData.happRelease.content.version }}</div>
                  <div><span style="font-weight: 600;">UI version: </span>{{ releaseData.guiRelease? releaseData.guiRelease.content.version : "no official UI" }}</div>
                  <!-- GUI version as well here? Needs to be fetched independently from a DevHub host -->
                </div>
                <span style="display: flex; flex: 1;"></span>
                <HCButton @click="() => {
                  $emit('release-selected', releaseDatas ? releaseDatas[0] : undefined);
                  close();
                }">Install</HCButton>
              </div>
            </div>

          </div>
        </div>

        <div v-else-if="releaseDatas && releaseDatas.length < 1" style="text-align: center;">
          There are no installable releases available for this app.
        </div>

        <div v-else-if="getReleaseDatasError" style="background: #f4b2b2; padding: 5px 10px; border-radius: 5px; width: 610px;">
          <b>Error getting releases from peer host:</b><br>
          {{ getReleaseDatasError }}
        </div>

        <div v-else class="column" style="align-items: center; width: 100%;">
          <mwc-circular-progress
            indeterminate
            style="margin-top: 30px; margin-bottom: 20px"
          ></mwc-circular-progress>
        </div>

      </div>


      <HCButton
        style="width: 80px; height: 30px; margin: 4px 6px; margin-bottom: 15px;"
        outlined
        @click="cancel"
      >Close
      </HCButton>

    </div>
  </HCDialog>

  <HCSnackbar
    :timeoutMs="10000"
    :labelText="snackbarText"
    ref="snackbar"
  ></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

import HCButton from "./subcomponents/HCButton.vue";
import HCDialog from "./subcomponents/HCDialog.vue";
import HCSnackbar from "./subcomponents/HCSnackbar.vue";

import { ResourceLocator, ReleaseData } from "../types";
import { AppEntry, Entity, GUIReleaseEntry, HappReleaseEntry, PublisherEntry } from "../appstore/types";
import { AppWebsocket, encodeHashToBase64 } from "@holochain/client";
import { APPSTORE_APP_ID } from "../constants";
import { collectBytes, fetchGuiReleaseEntry, getHappReleases, getHappReleasesFromHost, getPublisher, remoteCallToDevHubHost, tryWithHosts } from "../appstore/appstore-interface";
import { HappEntry } from "../devhub/types";

export default defineComponent({
  name: "SelectReleaseDialog",
  emits: ["error", "closing-dialog", "release-selected", "cancel"],
  components: {
    HCDialog,
    HCButton,
    HCSnackbar,
  },
  props: {
    app: {
      type: Object as PropType<AppEntry>,
      required: true,
    },
    appWebsocket: {
      type: Object as PropType<any>,
      required: true,
    },
    imgSrc: {
      type: String,
    },
  },
  data(): {
    showAdvanced: boolean;
    snackbarText: string | undefined;
    error: boolean;
    publisher: PublisherEntry | undefined;
    locale: string;
    releaseDatas: Array<ReleaseData> | undefined;
    getReleaseDatasError: string | undefined;
  } {
    return {
      showAdvanced: false,
      snackbarText: undefined,
      error: false,
      publisher: undefined,
      locale: "en-US",
      releaseDatas: undefined,
      getReleaseDatasError: undefined,
    };
  },
  mounted () {
    const customLocale = window.localStorage.getItem("customLocale");
    this.locale =  customLocale ? customLocale : navigator.language;
  },
  methods: {
    async getReleaseDatas(): Promise<void> {

      const appStoreInfo = await this.appWebsocket!.appInfo({
        installed_app_id: APPSTORE_APP_ID,
      });

      let happReleases: Array<Entity<HappReleaseEntry>> | undefined = undefined;

      const happLocator: ResourceLocator = {
        dna_hash: this.app.devhub_address.dna,
        resource_hash:  this.app.devhub_address.happ,
      }

      const devHubDnaHash = this.app.devhub_address.dna;

      // first try to get HappEntry to ensure that happ is available at all
      // otherwise cascade to another host.

      try {
        happReleases = await tryWithHosts(
          async (host) => {
            // first check whether HappEntry can be fetched since 'get_happ_releases'
            // also succeeds (but with an empty array) if the devhub host does not yet
            // have the HappEntry which would misleadingly end up suggesting that there
            // are no happ releases for that happ.
            const happEntry = await remoteCallToDevHubHost<Entity<HappEntry>>(
              this.appWebsocket as AppWebsocket,
              appStoreInfo,
              happLocator.dna_hash,
              host,
              "happ_library",
              "get_happ",
              { id: happLocator.resource_hash }
            );

            return getHappReleasesFromHost(
              this.appWebsocket as AppWebsocket,
              appStoreInfo,
              host,
              happLocator,
            )
          },
          this.appWebsocket as AppWebsocket,
          appStoreInfo,
          happLocator.dna_hash,
          "happ_library",
          "get_happ_releases",
        )
        // happReleases = await getHappReleases(this.appWebsocket as AppWebsocket, appStoreInfo, happLocator)
      } catch (e) {
        this.getReleaseDatasError = `Failed to find available releases: ${JSON.stringify(e)}`;
        console.error(`Failed to find available releases: ${JSON.stringify(e)}`)
        return;
      }

      if (!happReleases) {
        this.getReleaseDatasError = `Failed to find available releases: happReleases undefined.`;
        return;
      }

      console.log("@getReleaseDatas: got happReleases: ", happReleases);

      // this.selectedHappReleases = happReleases.map((entity) => entity.content).sort((a, b) => b.last_updated - a.last_updated);
      let releaseDatas: Array<ReleaseData> = [];

      console.log("@getReleaseDatas: fetching gui release entries...");

      try {
        await Promise.all(happReleases.map(
          async (happReleaseEntity) => {
            let releaseData: ReleaseData = {
              devhubDnaHash: devHubDnaHash,
              happRelease: happReleaseEntity,
              guiRelease: undefined,
            };

            const guiReleaseHash = happReleaseEntity.content.official_gui;
            if (guiReleaseHash) {
              const guiReleaseEntry = await tryWithHosts(
                (host) => remoteCallToDevHubHost<Entity<GUIReleaseEntry>>(
                  this.appWebsocket as AppWebsocket,
                  appStoreInfo,
                  devHubDnaHash,
                  host,
                  "happ_library",
                  "get_gui_release",
                  { id: guiReleaseHash }
                ),
                this.appWebsocket as AppWebsocket,
                appStoreInfo,
                devHubDnaHash,
                "happ_library",
                "get_gui_release",
              );

              releaseData.guiRelease = guiReleaseEntry;
            }

            releaseDatas.push(releaseData);
          })
        );
      } catch (e) {
        if (JSON.stringify(e).includes("No available peer host found")) {
          this.getReleaseDatasError = "No available peer host found.";
          return;
        } else {
          this.getReleaseDatasError = `Failed to find available releases: ${JSON.stringify(e)}`;
          console.error(`Failed to find available releases: ${JSON.stringify(e)}`)
          return;
        }
      }

      this.releaseDatas = releaseDatas.sort((a, b) => b.happRelease.content.published_at - a.happRelease.content.published_at);
      this.getReleaseDatasError = undefined;
    },
    async open() {
      (this.$refs.dialog as typeof HCDialog).open();

      let appStoreInfo = undefined;

      try {
        appStoreInfo = await this.appWebsocket!.appInfo({
          installed_app_id: APPSTORE_APP_ID,
        });
        this.publisher = await getPublisher(this.appWebsocket, appStoreInfo, this.app.publisher);
      } catch (e) {
        console.error(`Failed to get publisher info: ${JSON.stringify(e)}`)
      }

      await this.getReleaseDatas();

    },
    close() {
      (this.$refs.dialog as typeof HCDialog).close();
      this.$emit('closing-dialog');
    },
    cancel() {
      (this.$refs.dialog as typeof HCDialog).close();
      this.$emit('cancel');
    }
  },
});
</script>

<style scoped>

.advanced-button:hover {
  opacity: 0.8;
  cursor: pointer;
}


.card {
  min-width: 600px;
  background: #f6f6fa;
  border-radius: 15px;
  box-shadow: 0 0px 5px #9b9b9b;
  /* border: 2px solid #e1e1e1; */
  padding: 5px 15px 15px 15px;
  margin: 12px 0;
}
</style>

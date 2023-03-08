import { LitElement, css, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import {
  AppAgentWebsocket,
  ActionHash,
  AppAgentClient,
  CellId,
  AppInfo,
  ClonedCell,
  CellType,
  encodeHashToBase64,
} from '@holochain/client';
import { provide } from '@lit-labs/context';
import '@material/mwc-circular-progress';
import { v4 as uuidv4 } from "uuid";


import { clientContext } from './contexts';

import './clones/random/create-message';

@customElement('holochain-app')
export class HolochainApp extends LitElement {
  @state() loading = true;

  @provide({ context: clientContext })
  @property({ type: Object })
  client!: AppAgentClient;

  @state()
  appInfo: AppInfo | undefined;


  async firstUpdated() {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    this.client = await AppAgentWebsocket.connect('', 'launcher-tester');

    this.appInfo = await this.client.appInfo();

    this.loading = false;
  }

  async createRandomClone() {
    const uuid = uuidv4();
    const properties = {
      uuid,
    };

    const cloneName = uuid;

    // Create the We cell
    const clonedCell = await this.client.createCloneCell({
      role_name: "clones",
      modifiers: {
        properties,
        origin_time: Date.now(),
      },
      name: cloneName,
    });

    alert(`Created cloned cell: ${clonedCell.clone_id}`);

  }

  async createCloneFromInput() {
    const properties = {
      title: "Title",
      description: "Description",
      max_association_chars: null,
      max_reflection_chars: null,
      max_offer_chars: null,
      max_anecdote_chars: null,
    };

    const networkSeed = (this.shadowRoot?.getElementById("network-seed-input") as HTMLInputElement).value;
    const originTime = parseInt((this.shadowRoot?.getElementById("origin-time-input") as HTMLInputElement).value, 10);
    const cloneName = (this.shadowRoot?.getElementById("clone-name-input") as HTMLInputElement).value;

    const request = {
      role_name: "clones",
      modifiers: {
        properties,
        origin_time: originTime,
      },
      name: cloneName,
    };

    const clonedCell = await this.client.createCloneCell(request);

    console.log("Created clone cell with request: ", request);
    console.log("Generated DNA hash: ", encodeHashToBase64(clonedCell.cell_id[0]));
  }


  async disableClone(cellId: CellId) {
    await this.client.disableCloneCell({
      clone_cell_id: cellId,
    });

    await this.refresh();
  }

  async enableClone(cellId: CellId) {
    await this.client.enableCloneCell({
      clone_cell_id: cellId,
    });

    await this.refresh();
  }

  async refresh() {
    this.appInfo = await this.client.appInfo();
  }

  renderClones() {
    if (!this.appInfo) {
      return html`loading...`
    }
    const cellInfos = this.appInfo!.cell_info;

    console.log("cellInfos: ", cellInfos);

    const clonesCells = cellInfos.clones;

    const clonedCells = clonesCells
      .filter((cellInfo) => "cloned" in cellInfo)
      .map((cellInfo) => (cellInfo as { [CellType.Cloned]: ClonedCell }).cloned);




    return html`
      <div style="margin-top: 20px; border: 2px solid red; padding: 20px;">
        <div style="font-weight: bold">Attempt invalid zome call with unauthorized agent public key:</div>
        <create-message style="margin-bottom: 50px;" useFakeKey></create-message>
      </div>
      <div style="margin-top: 20px; border: 2px solid green; padding: 20px;">
        <div style="font-weight: bold">Create Message via valid zome call:</div>
        <create-message style="margin-bottom: 50px;"></create-message>
      </div>
      ${clonedCells.map((clonedCell) => html`
        <div style="display: flex; flex-direction: column; border-radius: 20px; margin: 50px; border: 1px solid black; padding: 10px;">
          <div>name: ${clonedCell.name}</div>
          <div>enabled: ${clonedCell.enabled}</div>
          <div
            @keypress=${async () => clonedCell.enabled
                ? this.disableClone(clonedCell.cell_id)
                : this.enableClone(clonedCell.cell_id)
            }
            @click=${async () => clonedCell.enabled
                ? this.disableClone(clonedCell.cell_id)
                : this.enableClone(clonedCell.cell_id)
            }
            style="text-decoratoin: underline; color: blue; cursor: pointer;"
          >${clonedCell.enabled ? "disable" : "enable"}</div>
        </div>
        `
      )}

    `

  }


  render() {
    if (this.loading)
      return html`
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      `;

    return html`
      <main>
        <h1>launcher tester</h1>

        <div style="margin-top: 20px; border: 2px solid black; padding: 20px;">
          <div style="font-weight: bold">Pick any file to check that the file chooser of the webview works:</div>
          <input type="file" />
        </div>
        <div style="margin-top: 20px; border: 2px solid black; padding: 20px;">
          <div style="font-weight: bold">Pick an image file to check that the file chooser of the webview works fine also with restrictions to file types:</div>
          <input type="file" accept="image/*"/>
        </div>

        <div style="margin-top: 20px; border: 2px solid black; padding: 20px;">
          <div style="font-weight: bold">Test different types of links:</div>
          <div><a href="#/test.html">This is a link to #/test.html</a></div>
          <div><a href="https://duckduckgo.com">This is a link to duckduckgo.com</a></div>
          <div><a href="tauri.localhost/test.html">This is a link to tauri.localhost/test.html</a></div>
          <div><a href="tauri.localhost/test.html">This is a link to tauri.localhost/test.html</a></div>
          <div
            @click=${() => window.open("https://duckduckgo.com")}
            @keypress=${() => window.open("https://duckduckgo.com")}
            style="cursor: pointer; font-weight: bold; text-decoration: underline;"
          >Click me to open DuckDuckGo via window.open()
          </div>
        </div>

        <div style="margin-top: 20px; border: 2px solid black; padding: 20px;">
          <div style="font-weight: bold">Create clone with custom input:</div>
          <label>Network Seed</label>
          <input id="network-seed-input" type="text">
          <label>Origin time</label>
          <input id="origin-time-input" type="number">
          <label>Clone Name</label>
          <input id="clone-name-input" type="text">

          <button @click=${this.createCloneFromInput}>Create Clone</button>
        </div>


        <div style="margin-top: 20px; border: 2px solid black; padding: 20px;">
          <div style="font-weight: bold">Create random clone:</div>
          <button @click=${() => this.createRandomClone()}>Create Random Clone</button>
        </div>
        <div id="content">
          ${this.renderClones()}
        </div>
      </main>
    `;
  }

  static styles = css`
    :host {
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: flex-start;
      font-size: calc(10px + 2vmin);
      color: #1a2b42;
      max-width: 960px;
      margin: 0 auto;
      text-align: center;
      background-color: var(--lit-element-background-color);
    }

    main {
      flex-grow: 1;
    }

    .app-footer {
      font-size: calc(12px + 0.5vmin);
      align-items: center;
    }

    .app-footer a {
      margin-left: 5px;
    }
  `;
}

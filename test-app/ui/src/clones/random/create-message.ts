import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AppAgentClient, fakeAgentPubKey, encodeHashToBase64, ProvisionedCell } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

import { clientContext } from '../../contexts';
import { Message } from './types';

@customElement('create-message')
export class CreateMessage extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;


  @state()
  _message: string | undefined;

  @property({ type: Boolean })
  useFakeKey = false;


  isMessageValid() {
    return true && this._message !== undefined;
  }

  async createMessage() {
    const message: Message = {
        message: this._message!,
    };

    try {

      if (this.useFakeKey) {
        const fakeKey = await fakeAgentPubKey();
        console.log("@create-message: real public key: ", encodeHashToBase64(this.client.myPubKey));
        console.log("@create-message: using fake public key for zome call: ", encodeHashToBase64(fakeKey));

        const appInfo = await this.client.appInfo();
        const cellInfos = appInfo.cell_info.clones;
        const cellInfo = (cellInfos.find((cell) => "provisioned" in cell)! as { "provisioned": ProvisionedCell}).provisioned;

        const record: Record = await this.client.callZome({
          cap_secret: null,
          cell_id: cellInfo.cell_id,
          zome_name: 'random',
          fn_name: 'create_message',
          payload: message,
          provenance: fakeKey, // use fake agent pubkey to test that Launcher properly rejects the zome call
        });

        alert("!! DANGER !!: Succeeded in making an invalid zome call with an unauthorized public key.");

        console.log("Used fake key: ", encodeHashToBase64(fakeKey));

      } else {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'clones',
          zome_name: 'random',
          fn_name: 'create_message',
          payload: message,
          // provenance: this.useFakeKey ? fakeKey : this.client.myPubKey, // use fake agent pubkey to test that Launcher properly rejects the zome call
        });

        alert("Successfully made valid zome call.");
        console.log("Received record back after zome call: ", record);
        console.log("actual public key: ", encodeHashToBase64(this.client.myPubKey));

        this.dispatchEvent(new CustomEvent('message-created', {
          composed: true,
          bubbles: true,
          detail: {
            messageHash: record.signed_action.hashed.hash
          }
        }));

      }
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      // throw new Error(e);
      throw new Error(`Error: ${e} \nStringified error: ${JSON.stringify(e)}`);
      // errorSnackbar.labelText = `Error creating the message: ${e.data.data}`;
      // errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Message</span>

          <div style="margin-bottom: 16px">
            <mwc-textarea outlined label="Message"  @input=${(e: CustomEvent) => { this._message = (e.target as any).value;} } required></mwc-textarea>
          </div>


        <mwc-button
          raised
          label=${this.useFakeKey ? "Create Message With Unauthorized Public Key": "Create Valid Message"}
          .disabled=${!this.isMessageValid()}
          @click=${() => this.createMessage()}
        ></mwc-button>
    </div>`;
  }
}

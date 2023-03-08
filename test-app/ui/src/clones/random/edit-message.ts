import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { ActionHash, EntryHash, AgentPubKey, Record, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
import { clientContext } from '../../contexts';
import { Message } from './types';

@customElement('edit-message')
export class EditMessage extends LitElement {

  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @property({
      hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  originalMessageHash!: ActionHash;

  
  @property()
  currentRecord!: Record;
 
  get currentMessage() {
    return decode((this.currentRecord.entry as any).Present.entry) as Message;
  }
 
  @state()
  _message!: string;


  isMessageValid() {
    return true && this._message !== undefined;
  }
  
  connectedCallback() {
    super.connectedCallback();
    this._message = this.currentMessage.message;
  }

  async updateMessage() {
    const message: Message = { 
      message: this._message!,
    };

    try {
      const updateRecord: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'clones',
        zome_name: 'random',
        fn_name: 'update_message',
        payload: {
          original_message_hash: this.originalMessageHash,
          previous_message_hash: this.currentRecord.signed_action.hashed.hash,
          updated_message: message
        },
      });
  
      this.dispatchEvent(new CustomEvent('message-updated', {
        composed: true,
        bubbles: true,
        detail: {
          originalMessageHash: this.originalMessageHash,
          previousMessageHash: this.currentRecord.signed_action.hashed.hash,
          updatedMessageHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the message: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit Message</span>
          <div style="margin-bottom: 16px">
          <mwc-textarea outlined label="Message" .value=${ this._message } @input=${(e: CustomEvent) => { this._message = (e.target as any).value;} } required></mwc-textarea>    
          </div>



        <div style="display: flex; flex-direction: row">
          <mwc-button
            outlined
            label="Cancel"
            @click=${() => this.dispatchEvent(new CustomEvent('edit-canceled', {
              bubbles: true,
              composed: true
            }))}
            style="flex: 1; margin-right: 16px"
          ></mwc-button>
          <mwc-button 
            raised
            label="Save"
            .disabled=${!this.isMessageValid()}
            @click=${() => this.updateMessage()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}

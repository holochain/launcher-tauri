import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { EntryHash, Record, ActionHash, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import './edit-message';

import { clientContext } from '../../contexts';
import { Message } from './types';

@customElement('message-detail')
export class MessageDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  messageHash!: ActionHash;

  _fetchRecord = new Task(this, ([messageHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'clones',
      zome_name: 'random',
      fn_name: 'get_message',
      payload: messageHash,
  }) as Promise<Record | undefined>, () => [this.messageHash]);

  @state()
  _editing = false;

  async deleteMessage() {
    try {
      await this.client.callZome({
        cap_secret: null,
        role_name: 'clones',
        zome_name: 'random',
        fn_name: 'delete_message',
        payload: this.messageHash,
      });
      this.dispatchEvent(new CustomEvent('message-deleted', {
        bubbles: true,
        composed: true,
        detail: {
          messageHash: this.messageHash
        }
      }));
      this._fetchRecord.run();
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('delete-error') as Snackbar;
      errorSnackbar.labelText = `Error deleting the message: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  renderDetail(record: Record) {
    const message = decode((record.entry as any).Present.entry) as Message;

    return html`
      <mwc-snackbar id="delete-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
          <span style="font-size: 18px; flex: 1;">Message</span>

          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
          <mwc-icon-button style="margin-left: 8px" icon="delete" @click=${() => this.deleteMessage()}></mwc-icon-button>
        </div>

        <div style="display: flex; flex-direction: row; margin-bottom: 16px">
	  <span><strong>Message</strong></span>
 	  <span style="white-space: pre-line">${ message.message }</span>
        </div>

      </div>
    `;
  }
  
  renderMessage(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested message was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-message
    	  .originalMessageHash=${this.messageHash}
    	  .currentRecord=${maybeRecord}
    	  @message-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-message>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderMessage(maybeRecord),
      error: (e: any) => html`<span>Error fetching the message: ${e.data.data}</span>`
    });
  }
}

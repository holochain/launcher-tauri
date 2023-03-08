import test from 'node:test';
import assert from 'node:assert';

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource } from '@holochain/client';
import { decode } from '@msgpack/msgpack';


async function sampleMessage(cell: CallableCell, partialMessage = {}) {
    return {
        ...{
	  message: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialMessage
    };
}

export async function createMessage(cell: CallableCell, message = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "random",
      fn_name: "create_message",
      payload: message || await sampleMessage(cell),
    });
}

test('create Message', { concurrency: 1 }, async t => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/launcher-tester.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Message
    const record: Record = await createMessage(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read Message', { concurrency: 1 }, async t => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/launcher-tester.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleMessage(alice.cells[0]);

    // Alice creates a Message
    const record: Record = await createMessage(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created Message
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "random",
      fn_name: "get_message",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update Message', { concurrency: 1 }, async t => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/launcher-tester.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Message
    const record: Record = await createMessage(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the Message
    let contentUpdate: any = await sampleMessage(alice.cells[0]);
    let updateInput = {
      original_message_hash: originalActionHash,
      previous_message_hash: originalActionHash,
      updated_message: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "random",
      fn_name: "update_message",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated Message
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "random",
      fn_name: "get_message",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the Message again
    contentUpdate = await sampleMessage(alice.cells[0]);
    updateInput = { 
      original_message_hash: originalActionHash,
      previous_message_hash: updatedRecord.signed_action.hashed.hash,
      updated_message: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "random",
      fn_name: "update_message",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated Message
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "random",
      fn_name: "get_message",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test('create and delete Message', { concurrency: 1 }, async t => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/launcher-tester.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a Message
    const record: Record = await createMessage(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the Message
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "random",
      fn_name: "delete_message",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);
        
    // Bob tries to get the deleted Message
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "random",
      fn_name: "get_message",
      payload: record.signed_action.hashed.hash,
    });
    assert.equal(readDeletedOutput, undefined);
  });
});

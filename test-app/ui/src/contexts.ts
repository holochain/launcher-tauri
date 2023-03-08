import { createContext } from '@lit-labs/context';
import { AppAgentClient } from '@holochain/client';

export const clientContext = createContext<AppAgentClient>('appAgentClient');


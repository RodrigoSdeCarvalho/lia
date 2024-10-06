import React, { useState } from 'react';
import tauriApi from '@tauri-apps/api';
const { invoke } = tauriApi.core;

import { listen } from '@tauri-apps/api/event';

function CommandRunner({ selectedFolder, addLog }) {
  const [commandName, setCommandName] = useState('');

  const runCommand = async () => {
    if (!selectedFolder) {
      alert('Please select a folder first.');
      return;
    }

    try {
      // Listen for logs
      listen('log', (event) => {
        addLog(event.payload);
      });

      await invoke('run_command', { name: commandName, path: selectedFolder });
    } catch (error) {
      console.error('Error running command:', error);
    }
  };

  return (
    <div className="mt-4">
      <h2 className="text-xl mb-2">Run Command</h2>
      <input
        type="text"
        placeholder="Command Name"
        value={commandName}
        onChange={(e) => setCommandName(e.target.value)}
        className="border p-2 rounded w-full mb-2"
      />
      <button
        onClick={runCommand}
        className="px-4 py-2 bg-blue-500 text-white rounded"
      >
        Run Command
      </button>
    </div>
  );
}

export default CommandRunner;

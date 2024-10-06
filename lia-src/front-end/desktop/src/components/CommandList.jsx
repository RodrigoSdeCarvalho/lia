import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api ';

function CommandList() {
  const [commands, setCommands] = useState([]);

  useEffect(() => {
    fetchCommands();
  }, []);

  const fetchCommands = async () => {
    try {
      const result = await invoke('list_commands', { limit: 100, offset: 0 });
      setCommands(result);
    } catch (error) {
      console.error('Error fetching commands:', error);
    }
  };

  return (
    <div>
      <h2 className="text-xl mb-2">Stored Commands</h2>
      <ul className="bg-white shadow rounded p-2 max-h-64 overflow-y-auto">
        {commands.map((cmd, index) => (
          <li key={index} className="border-b py-2">
            <p>
              <strong>{cmd.name}</strong>
            </p>
            <p>{cmd.description}</p>
            <p className="text-sm text-gray-600">{cmd.command_text}</p>
            <p className="text-sm text-gray-500">Tags: {cmd.tags?.join(', ')}</p>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default CommandList;

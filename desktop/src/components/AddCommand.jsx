import React, { useState } from 'react';
import tauriApi from '@tauri-apps/api';
const { invoke } = tauriApi.core;

function AddCommand() {
  const [name, setName] = useState('');
  const [commandText, setCommandText] = useState('');
  const [description, setDescription] = useState('');
  const [tags, setTags] = useState('');

  const addNewCommand = async () => {
    try {
      const tagsArray = tags ? tags.split(',').map((tag) => tag.trim()) : [];
      await invoke('add_command', {
        command: {
          name,
          command_text: commandText,
          description,
          tags: tagsArray,
        },
      });
      alert('Command added successfully.');
      // Reset form
      setName('');
      setCommandText('');
      setDescription('');
      setTags('');
    } catch (error) {
      console.error('Error adding command:', error);
      alert('Failed to add command.');
    }
  };

  return (
    <div>
      <h2 className="text-xl mb-2">Add New Command</h2>
      <input
        type="text"
        placeholder="Command Name"
        value={name}
        onChange={(e) => setName(e.target.value)}
        className="border p-2 rounded w-full mb-2"
      />
      <textarea
        placeholder="Command Text"
        value={commandText}
        onChange={(e) => setCommandText(e.target.value)}
        className="border p-2 rounded w-full mb-2"
      ></textarea>
      <input
        type="text"
        placeholder="Description"
        value={description}
        onChange={(e) => setDescription(e.target.value)}
        className="border p-2 rounded w-full mb-2"
      />
      <input
        type="text"
        placeholder="Tags (comma-separated)"
        value={tags}
        onChange={(e) => setTags(e.target.value)}
        className="border p-2 rounded w-full mb-2"
      />
      <button
        onClick={addNewCommand}
        className="px-4 py-2 bg-green-500 text-white rounded"
      >
        Add Command
      </button>
    </div>
  );
}

export default AddCommand;

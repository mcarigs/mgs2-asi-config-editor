import { ref, watchEffect, Ref } from 'vue';
import { invoke } from '@tauri-apps/api';

// Define the structure of INI data
interface IniData {
  [section: string]: {
    [key: string]: string;
  };
}

// Composable function to handle INI file operations
export function useIniFile(filePath: Ref<string>) {
  // Reactive reference to store the loaded INI data
  const iniData: Ref<IniData | null> = ref(null);

  // Function to load an INI file
  const loadIniFile = async (path: string) => {
    try {
      // Invoke the Tauri command to read the INI file
      const result = await invoke<IniData>('read_ini_file', { path });
      // Update the iniData ref with the loaded data
      iniData.value = result;
    } catch (error) {
      console.error('Failed to load INI file:', error);
    }

    try {
      // Invoke the Tauri command to ensure all necessary INI files exist
      await invoke('ensure_ini_files_exist');
      console.log('INI files ensured');
    } catch (error) {
      console.error('Failed to ensure INI files exist:', error);
    }
  };

  // Watch for changes in the filePath and reload the INI file when it changes
  watchEffect(() => {
    if (filePath.value) {
      loadIniFile(filePath.value);
    }
  });

  // Return the iniData ref and the loadIniFile function
  return {
    iniData,
    loadIniFile
  };
}

import { ref, computed, Ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

// Define the structure of INI data
interface IniData {
  [section: string]: {
    [key: string]: string;
  };
}

// Composable function to handle INI file editing
export function useEditIniFile(iniData: Ref<IniData | null>, filePath: Ref<string>) {
  // Reactive reference to store the edited INI data
  const editedData: Ref<IniData | null> = ref(null);
  // Reactive reference to track editing state
  const isEditing = ref(false);

  // Function to start editing
  const startEditing = () => {
    // Create a deep copy of the original data
    editedData.value = iniData.value ? JSON.parse(JSON.stringify(iniData.value)) : null;
    isEditing.value = true;
  };

  // Function to cancel editing
  const cancelEditing = () => {
    editedData.value = null;
    isEditing.value = false;
  };

  // Function to remove empty sections from the data
  const cleanData = (data: IniData): IniData => {
    return Object.fromEntries(
      Object.entries(data).filter(([_, section]) => Object.keys(section).length > 0)
    ) as IniData;
  };

  // Function to save the edited INI file
  const saveIniFile = async (): Promise<boolean> => {
    if (!editedData.value) return false;

    try {
      // Invoke the Tauri command to write the INI file
      await invoke('write_ini_file', { path: filePath.value, data: editedData.value });
      // Update the original data with the edited data
      iniData.value = JSON.parse(JSON.stringify(editedData.value));
      isEditing.value = false;
      return true; // Indicate successful save
    } catch (error) {
      console.error('Error saving INI file:', error);
      return false; // Indicate failed save
    }
  };

  // Function to update a specific value in the edited data
  const updateEditedData = (section: string, key: string, value: string) => {
    if (editedData.value && editedData.value[section]) {
      editedData.value[section][key] = value;
    }
  };

  // Computed property to sort the INI data
  const sortedIniData = computed(() => {
    const dataToSort = isEditing.value ? editedData.value : iniData.value;
    if (!dataToSort) return null;

    const cleanedData = cleanData(dataToSort);

    // Sort sections alphabetically and their keys with non-empty values first
    return Object.fromEntries(
      Object.entries(cleanedData)
        .sort(([a], [b]) => a.localeCompare(b))
        .map(([sectionName, section]) => [
          sectionName,
          Object.fromEntries(
            Object.entries(section)
              .sort(([keyA, valueA], [keyB, valueB]) => {
                // If both values are empty or both are non-empty, sort alphabetically
                if ((valueA === '' && valueB === '') || (valueA !== '' && valueB !== '')) {
                  return keyA.localeCompare(keyB);
                }
                // If valueA is empty and valueB is not, valueA should come last
                if (valueA === '' && valueB !== '') {
                  return 1;
                }
                // If valueA is not empty and valueB is, valueA should come first
                if (valueA !== '' && valueB === '') {
                  return -1;
                }
                // This line should never be reached, but TypeScript might expect a return here
                return 0;
              })
          )
        ])
    ) as IniData;
  });

  // Return the composable's functions and reactive data
  return {
    editedData,
    isEditing,
    sortedIniData,
    startEditing,
    cancelEditing,
    saveIniFile,
    updateEditedData
  };
}

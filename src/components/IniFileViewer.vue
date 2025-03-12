<script setup>
  import { computed, onMounted, watch, ref } from 'vue';
  import { useRoute } from 'vue-router';
  import { useIniFile } from '../composables/loadIniFile';
  import { useEditIniFile } from '../composables/editIniFile';
  import { confirm } from '@tauri-apps/api/dialog';

  // Define props for the component
  const props = defineProps({
    fileName: {
      type: String,
      required: true
    }
  });

  const confirmEditCancel = async () => {
    return await confirm('You have unsaved changes. Are you sure you want to navigate away?', { title: 'Unsaved Changes', type: 'warning' });
  }

  // Get the current route
  const route = useRoute();

  // Compute the file path based on the fileName prop
  const filePath = computed(() => `${props.fileName}`);

  // Use the loadIniFile composable to get and load INI data
  const { iniData, loadIniFile } = useIniFile(filePath.value);

  // Use the editIniFile composable to handle editing functionality
  const {
    editedData,
    isEditing,
    sortedIniData,
    startEditing,
    cancelEditing: originalCancelEditing,
    saveIniFile,
    updateEditedData
  } = useEditIniFile(iniData, filePath);

  // Ref to store save status message
  const saveStatus = ref('');

  // Modify the cancelEditing function to reset editedData
  const cancelEditing = () => {
    originalCancelEditing();
    editedData.value = null;
  };

  // Handle save action
  const handleSave = async () => {
    saveStatus.value = 'Saving...';
    console.log('Saving file:', filePath.value);
    console.log('Data to save:', editedData.value);

    const success = await saveIniFile();
    console.log('Save result:', success);

    if (success) {
      saveStatus.value = 'Saved successfully!';
    } else {
      saveStatus.value = 'Save failed. Please try again.';
    }
    setTimeout(() => { saveStatus.value = ''; }, 3000);
  };

  // Watch for changes in fileName and reload INI data
  watch(() => props.fileName, async (newFileName, oldFileName) => {
    if (isEditing.value && newFileName !== oldFileName) {
    try {
      const confirmNavigation = await confirm(
        "You have unsaved changes. Are you sure you want to navigate away?",
        { title: "Confirm Navigation", type: "warning" }
      );

      console.log(`confirmNavigation: ${confirmNavigation}`);

      if (confirmNavigation) {
        cancelEditing();
        await loadIniFile(`${newFileName}`);
      } else {
        // If the user cancels navigation, we need to revert the route change
        history.pushState(history.state, '', `#/${oldFileName}`);
        await loadIniFile(`${oldFileName}`);
      }
      } catch (error) {
        console.error("Error during navigation confirmation:", error);
        // Handle the error appropriately
      }
    } else {
        await loadIniFile(`${newFileName}`);
    }
  }, { immediate: true });

  // Helper functions to determine value types
  const isBooleanValue = (value) => {
    const booleanValues = ['true', 't', 'yes', 'y', 'on', 'false', 'f', 'no', 'n', 'off'];
    return booleanValues.includes(value.toLowerCase());
  };

  const isTrueValue = (value) => {
    const trueValues = ['true', 't', 'yes', 'y', 'on'];
    return trueValues.includes(value.toLowerCase());
  };

  const isColorKey = (key) => {
    return key === 'TextColor' || key === 'TextOutline';
  };

  const isMultilineValue = (value) => {
    return value.includes('\n');
  };

  // Mapping of multi-choice values associated with their respective keys
  const multiChoiceOptions = {
    AlertLevel: [
      { value: 'Infiltration', display: 'Infiltration' },
      { value: 'Caution', display: 'Caution' },
      { value: 'Evasion', display: 'Evasion' },
      { value: 'Alert', display: 'Alert' },
    ],
    HostageHour: [
      { value: '', display: 'Based on clock' },
      { value: '0', display: 'Jennifer' },
      { value: '13', display: 'Beasts' },
      { value: '22', display: 'Old Beauties'},
      {value: '23', display: 'Normal Mix'}
    ],
    Mode: [
      { value: 'Basic', display: 'Basic' },
      { value: 'Custom', display: 'Custom' },
      { value: 'Extreme', display: 'Extreme' }
    ],
    Precision: [
      { value: '0', display: '0' },
      { value: '1', display: '0.0' },
      { value: '2', display: '0.00' }
    ],
    Priority: [
      { value: '0', display: 'Low' },
      { value: '1', display: 'Below Normal' },
      { value: '2', display: 'Normal' },
      { value: '3', display: 'Above Normal' },
      { value: '4', display: 'High' },
    ],
    Reserve: [
      { value: 'Previous', display: 'Previous' },
      { value: 'Unequip', display: 'Unequip' },
      { value: 'Reserve', display: 'Normal' },
      { value: 'NoChange', display: 'NoChange' }
    ],
    Weapon: [
      { value: 'M9', display: 'M9' },
      { value: 'SOCOM', display: 'SOCOM' },
      { value: 'AKs-74u', display: 'AKs-74u' },
      { value: 'M4', display: 'M4' },
      { value: 'Grenade', display: 'Grenade' },
      { value: 'Magazine', display: 'Magazine' },
      { value: 'Nikita', display: 'Nikita' },
      { value: 'RGB6', display: 'RGB6' },
      { value: 'Stinger', display: 'Stinger' },
      { value: 'Claymore', display: 'Claymore' },
      { value: 'Book', display: 'Book' },
      { value: 'C4', display: '' },
      { value: 'Coolant', display: 'Coolant' },
      { value: 'Stun Grenade', display: 'Stun Grenade' },
      { value: 'Chaff Grenade', display: 'Chaff Grenade' },
      { value: 'PSG1-1', display: 'PSG-1' },
      { value: 'PSG1-T', display: 'PSG-1T' }
    ],
    Item: [
      { value: 'Ration', display: 'Ration' },
      { value: 'Pentazemin', display: 'Pentazemin' },
      { value: 'Medicine', display: 'Medicine' },
      { value: 'Bandage', display: 'Bandage' },
      { value: 'Thermal Goggles', display: ' Goggles' },
      { value: 'NVG', display: 'NVG' },
      { value: 'Mine Detector', display: 'Mine Detector' },
      { value: 'Sensor A', display: 'Sensor A' },
      { value: 'Sensor B', display: 'Sensor B' },
      { value: 'AP Sensor', display: 'AP Sensor' },
      { value: 'Camera', display: 'Camera' },
      { value: 'Scope', display: 'Scope' },
      { value: 'Digital Camera', display: 'Digital Camera' },
      { value: 'Box 1', display: 'Box 1' },
      { value: 'Box 2', display: 'Box 2' },
      { value: 'Box 3', display: 'Box 3' },
      { value: 'Box 4', display: 'Box 4' },
      { value: 'Box 5', display: 'Box 5' },
      { value: 'Wet Box', display: 'Wet Box' },
      { value: 'Body Armor', display: 'Body Armor' },
      { value: 'BDU', display: 'BDU' },
      { value: 'Bandana', display: 'Bandana' },
      { value: 'Infinity Wig', display: 'Infinity Wig' },
      { value: 'Blue Wig', display: 'Blue Wig' },
      { value: 'Orange Wig', display: 'Orange Wig' },
      { value: 'Wig C', display: 'Wig C' },
      { value: 'Wig D', display: 'Wig D' },
      { value: 'Stealth', display: 'Stealth' },
      { value: 'Bandana', display: 'Bandana' },

    ]
  };

  const getDisplayName = (key, value) => {
    const option = multiChoiceOptions[key].find(opt => opt.value === value);
    return option ? option.display : value;
  };

  const isMultiChoiceValue = (key) => {

  return key in multiChoiceOptions;
  };

  // Log initial and updated INI data
  onMounted(() => {
    console.log('Initial iniData:', iniData.value);
  });

  watch(iniData, (newData) => {
    console.log('Updated iniData:', newData);
  });
</script>

<template>
  <div class="ini-file-viewer">
    <h1>{{ fileName }}</h1>
    <!-- Controls for editing -->
    <div class="controls">
      <button v-if="!isEditing" @click="startEditing">Edit</button>
      <template v-else>
        <button @click="handleSave">Save</button>
        <button @click="cancelEditing">Cancel</button><br>
        <span class="save-status">{{ saveStatus }}</span>
      </template>
    </div>
    <!-- INI content display -->
    <div v-if="sortedIniData" class="ini-content">
      <div v-for="(section, secName) in sortedIniData" :key="secName" class="ini-section">
        <!-- Section Header -->
        <template v-if="Object.keys(section).length > 0">
          <h3 class="section-header">{{ secName }}</h3>
          <!-- Section Content-->
          <div class="section-content">
            <div v-for="(value, key) in section" :key="key" class="ini-item">
              <div class="key-colon">
                <span class="key">{{ key }}</span>
                <span class="colon">:</span>
              </div>
              <div class="value">
                <!-- Color picker for color values -->
                <template v-if="isColorKey(key)">
                  <input
                    v-if="isEditing"
                    type="color"
                    :value="'#' + value"
                    @input="e => updateEditedData(secName, key, e.target.value.slice(1))"
                  />
                  <div
                    v-else
                    class="color-preview"
                    :style="{ backgroundColor: '#' + value }"
                  ></div>
                </template>
                <!-- Checkbox for boolean values -->
                <template v-else-if="isBooleanValue(value)">
                  <input
                    type="checkbox"
                    :checked="isTrueValue(value)"
                    :disabled="!isEditing"
                    @change="e => updateEditedData(secName, key, e.target.checked ? 'Yes' : 'No')"
                  />
                </template>
                <!-- Multiple choice values-->

                <template v-else-if="isMultiChoiceValue(key)">
                  <select
                    v-if="isEditing"
                    :value="value"
                    @change="e => updateEditedData(secName, key, e.target.value)"
                  >
                    <option v-for="option in multiChoiceOptions[key]" :key="option.value" :value="option.value">
                      {{ option.display }}
                    </option>
                  </select>
                  <span v-else>{{ getDisplayName(key, value) }}</span>
                </template>
                <!-- Textarea for multiline values -->
                <template v-else-if="isMultilineValue(value)">
                  <textarea
                    :name="key"
                    :disabled="!isEditing"
                    :cols="5"
                    :rows="5"
                    @input="e => updateEditedData(secName, key, e.target.value)"
                  >{{ value }}</textarea>
                </template>
                <!-- Text input for other values -->
                <template v-else>
                  <input
                    v-if="isEditing"
                    :value="value"
                    @input="e => updateEditedData(secName, key, e.target.value)"
                  />
                  <span v-else>{{ value }}</span>
                </template>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<style scoped>
  /* Styles for the INI file viewer */
  .ini-file-viewer {
    width: 100%;
    max-width: none;
    margin: 0;
    padding: 20px;
    box-sizing: border-box;
  }

  /* Styles for control buttons */
  .controls {
    margin-top: -10px;
    margin-bottom: 20px;
    text-align: center;
  }

  .save-status {
    margin-left: 10px;
    font-style: italic;
  }

  /* Container holding INI content */
  .ini-content {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
  }

  /* Outer box surrounding INI sections
     and their key-value pairs */
  .ini-section {
    margin-bottom: 20px;
    margin-left: 10px;
  }

  /* Responsive layout for smaller screens */
  @media (max-width: 1200px) {
    .ini-section {
      width: 100%;
      background-color: #6a6a6a;
    }
  }

  /* Header bar displaying INI section name */
  .section-header {
    text-align: center;
    background-color: #4286f465;
    padding: 5px;
    margin: 0;
  }

  /* Container with key-value pairs */
  .section-content {
    padding: 20px;
  }

  .key {
    text-align: center;
    padding-left: 120px;
  }

  .key-colon {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    width: 50%;
    padding-right: 5px;
  }

  /* Styles for individual INI items */
  .ini-item {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    margin-bottom: 5px;
  }

  .colon {
    margin-left: 1px;
  }

  .value {
    width: 50%;
    padding-left: 5px;
  }

  /* Styles for input elements */
  input {
    width: 35%;
    padding: 3px 6px;
  }

  input[type="checkbox"] {
    width: auto;
    margin-left: 0;
  }

  input[type="color"] {
    width: 40px;
    height: 20px;
    padding: 0;
    border: none;
  }

  .color-preview {
    width: 40px;
    height: 20px;
  }

  /* Button styles */
  button {
    margin-right: 10px;
    padding: 5px 10px;
    background-color: #4285f4;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 20px;
    font-weight: 400;
  }

  button:hover {
    background-color: #2851ab;
  }

  h1 {
    text-align: center;
    margin-top: 0px;
    margin-bottom: 20px;
  }

  /* Textarea styles */
  textarea {
    width: 35%;
    min-height: 80px;
    max-height: 100px;
    resize: none;
  }

  select {
    width: auto;
    padding: 2px 5px;
  }

  /* Styles for preformatted text */
  pre {
    white-space: pre-wrap;
    word-wrap: break-word;
    background-color: rgba(255, 255, 255, 0.1);
    padding: 5px;
    border-radius: 4px;
  }
</style>

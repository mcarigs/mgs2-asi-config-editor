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
    return await confirm('You have unsaved changes. Are   you sure you want to navigate away?', { title: 'Unsaved Changes', type: 'warning' });
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
        await loadIniFile(`.editordata/${newFileName}`);
      } else {
        // If the user cancels navigation, we need to revert the route change
        // This assumes you're using vue-router. Adjust if you're using a different routing solution.
        history.pushState(history.state, '', `#/${oldFileName}`);
        await loadIniFile(`.editordata/${oldFileName}`);
      }
    } catch (error) {
      console.error("Error during navigation confirmation:", error);
      // Handle the error appropriately
    }
  } else {
    await loadIniFile(`.editordata/${newFileName}`);
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
    margin-bottom: 20px;
    text-align: center;
  }

  .save-status {
    margin-left: 10px;
    font-style: italic;
  }

  /* Layout for INI content */
  .ini-content {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
  }

  .ini-section {
    width: calc(50% - 10px);
    margin-bottom: 20px;
  }

  /* Responsive layout for smaller screens */
  @media (max-width: 1200px) {
    .ini-section {
      width: 100%;
      background-color: #6a6a6a;
    }
  }

  /* Styles for section headers and content */
  .section-header {
    text-align: center;
    background-color: rgba(150, 146, 146, 0.518);
    padding: 10px;
    margin: 0;
  }

  .section-content {
    padding: 15px;
  }

  /* Styles for individual INI items */
  .ini-item {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    margin-bottom: 10px;
  }

  .key-colon {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    width: 50%;
    padding-right: 10px;
  }

  .key {
    text-align: right;
  }

  .colon {
    margin-left: 5px;
  }

  .value {
    width: 50%;
    padding-left: 10px;
  }

  /* Styles for input elements */
  input {
    width: 15%;
    padding: 2px 5px;
  }

  input[type="checkbox"] {
    width: auto;
    margin-left: 0;
  }

  input[type="color"] {
    width: 50px;
    height: 25px;
    padding: 0;
    border: none;
  }

  .color-preview {
    width: 50px;
    height: 25px;
    border: 1px solid #ccc;
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
  }

  button:hover {
    background-color: #3367d6;
  }

  h1 {
    text-align: center;
    margin-bottom: 20px;
  }

  /* Textarea styles */
  textarea {
    width: 50%;
    height: 60px;
    min-height: 80px;
    max-height: 100px;
    resize: none;
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

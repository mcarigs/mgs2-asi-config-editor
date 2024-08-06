import { message } from '@tauri-apps/api/dialog';
import { onMounted } from 'vue';

// Define an enum for different types of messages
enum MessageType {
  info = "info",
  warning = "warning",
  error = "error",
};

// Function to show a dialog message
export function showDialog(msg: string, msgTitle: string, msgType: MessageType) {
  // Define an async function to show the message dialog
  const showMessageDialog = async () => {
    try {
      // Use Tauri's message function to display the dialog
      await message(msg, { title: msgTitle, type: msgType });
    } catch (error) {
      // Log any errors that occur when trying to display the dialog
      console.error('Failed to display message dialog:', error);
    }
  };

  // Use Vue's onMounted hook to show the dialog when the component is mounted
  onMounted(() => {
    showMessageDialog();
  });
}

// Export the MessageType enum as the default export
export default MessageType;

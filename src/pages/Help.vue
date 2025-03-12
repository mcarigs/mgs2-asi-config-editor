<script setup lang="ts">
// Add any necessary imports or setup logic here
</script>

<template>
  <div class="help-page">
    <div class="help-container">
      <!-- Configuring Mods Section -->
      <h1>Configuring Mods</h1>
      <!-- List of configuration instructions for mods -->
      <ul>
        <li>To disable a mod, check the <code>Enabled</code> checkbox on that mod's page</li>
        <li>Some mods can be toggled on the fly. To do this, check the <code>Enabled</code> checkbox on the mod's page,
          then check the <code>Active</code> checkbox (active on launch) or leave it unchecked (not active on launch)
          and set up a toggle shortcut (see below) in the <code>[ModuleName.Toggle]</code> section</li>
        <li><strong>TextColor/TextOutlines</strong> are in the hex format <code>RRGGBB</code></li>
          <ul><li>This editor uses a color picker widget so configuring these options is much more simple</li></ul>
        <li>For <strong>TextPosX</strong>, the far left is <code>0</code>, far right is <code>640</code></li>
        <li>For <strong>TextPosY</strong>, the top is <code>0</code>, bottom is <code>480</code></li>
        <li>For <strong>TextAlign</strong>, <code>0</code> is left align, <code>1</code> is right align, <code>2</code> is centred</li>
        <li>In TextFormat and TextContent, you may see codes such as {}, {Score} or {Angle:+011.4f}.
          These are used to insert a provided value, formatted in a particular way</li>
          <ul>
            <li>If more than one value is available, all the values will be listed in a comment near the setting itself</li>
            <li>These formats follow the fmt syntax <a href="https://fmt.dev/latest/syntax.html">https://fmt.dev/latest/syntax.html</a>, which is similar to the C printf syntax <a href="https://cplusplus.com/reference/cstdio/printf/">https://cplusplus.com/reference/cstdio/printf/</a></li>
          </ul>
      </ul>

      <hr>

      <!-- Shortcuts Section -->
      <h1>Shortcuts</h1>
      <!-- Description of module shortcuts -->
      <p>Many of the modules here have shortcuts which allow you to trigger things inside the module. You'll see categories such as <code>[NameOfModule.Toggle]</code>, where "Toggle" is usually a description of what the shortcut does.</p>

      <p>Shortcuts have four available options (you can have any or all of them):</p>
      <!-- List of shortcut types -->
      <ul>
          <li>Keyboard is a keyboard input (e.g. CtrlShift+D)</li>
          <li>Gamepad is a gamepad input (e.g. L1+Circle)</li>
          <li>ContextMenu is the name of an item that will appear in the game's context menu</li>
          <li>ContextMenuPosition is an optional number to set where the item appears in the context menu</li>
          <li>You can also have multiple Keyboard/Gamepad lines if you want multiple shortcuts</li>
      </ul>

      <!-- Keyboard Shortcuts Subsection -->
      <h2>Keyboard</h2>
      <p>Keyboard is a string representing a keyboard shortcut. It's made up of:</p>
      <!-- List of keyboard shortcut components -->
      <ul>
          <li>0 or more modifier (Ctrl/Alt/Shift) keys, represented by e.g. CS, CtrlShift, ControlShift, etc.</li>
          <li>A separator (either symbol + or #)</li>
          <li>A single key to trigger the shortcut</li>
          <li>If + is used as the separator, you can enter A-Z, 0-9, N0-N9 (numpad keys), or F1-F24 as your key</li>
          <li>If # is used instead, the key is a 2-digit hexadecimal key code
              (<a href="https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes">see here for a list</a>)</li>
      </ul>

      <!-- Examples of keyboard shortcuts -->
      <h3>Examples:</h3>
      <pre>
        A (or +A)   = the A key on its own
        #41         = also the A key on its own
        S+F1        = Shift+F1
        Shift+F1    = insert thinking emoji here
        CA#2E       = Ctrl+Alt+Del (not recommended!)
        CtrlAlt#2E  = same
      </pre>
      <p>The modifier keys MUST be a single word such as CtrlAlt, no symbols like Ctrl+Alt!<br>
      Capitalisation (or lack of it) doesn't matter though</p>

      <hr>

      <!-- Gamepad Shortcuts Subsection -->
      <h2>Gamepad</h2>
      <p>Gamepad is a string representing a gamepad shortcut. It's made up of:</p>
      <!-- List of gamepad shortcut components -->
      <ul>
          <li>a series of button names (see "Valid input names" below), each separated by the symbol +</li>
          <li>(optional) the symbol | followed by a single button name</li>
      </ul>

      <!-- Explanation of shortcut activation -->
      <p>If only + signs are used, it doesn't matter which button is the last to be pressed -<br>
        once all the buttons are pressed at once the shortcut will activate<br>
      If a | is also used, the shortcut will only activate when that specific button is pressed<br>
        while holding all the other buttons</p>

      <!-- Examples of gamepad shortcuts -->
      <p>Example:</p>
      <pre>
        L1+R1+L2+R2+Start - activates as soon as all 5 buttons are held
        L1+R1+L2+R2|Start - activates when Start is pressed (while all the shoulder buttons are held)
      </pre>

      <!-- List of valid gamepad input names -->
      <p>Valid input names:</p>
      <pre>
        Directions:       Up/U, Right/R, Down/D, Left/L
        Face buttons:     Triangle/Tri/T, Circle/O, Cross/X, Square/Sq, Select/Sel, Start, L3, R3
        Shoulder buttons: L1, R1, L2, R2
      </pre>

      <hr>

      <!-- Context Menu Shortcuts Subsection -->
      <h2>ContextMenu</h2>
      <p>ContextMenu is a name that will be turned into a clickable shortcut in the game's context menu.<br>
        Open the context menu by right-clicking on the game, or pressing the Menu keyboard key<br>
          (between AltGr and RightCtrl).</p>

      <!-- Explanation of ContextMenuPosition -->
      <p>ContextMenuPosition is an optional number to set where the item appears in the context menu<br>
        The default is 0<br>
        Negative numbers (e.g. -1) appear higher - the more negative, the higher<br>
        Positive numbers (e.g. 1) appear lower - the more positive, the lower</p>

      <hr>

      <!-- Mod Descriptions Section -->
      <h1>Mod Descriptions</h1>
      <div class="help-content">
        <table class="help-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Description</th>
            </tr>
          </thead>
          <!-- Table rows for each mod description -->
          <tbody>
            <tr>
              <td>Actions</td>
              <td>Launch applications such as MGS2 Trainer directly from the game</td>
            </tr>
            <tr>
              <td>Affinity/Priority Changer</td>
              <td>Set which CPU cores the game is assigned to, and its CPU priority</td>
            </tr>
            <tr>
              <td>Ames</td>
              <td>Set the type of hostages when searching for Ames, and practice finding him with a repeat-search mode.
              </td>
            </tr>
            <tr>
              <td>Caution%</td>
              <td>Challenge mode with a neverending Caution. Or, start every area in Evasion, or pick a random alert
                state for every area.</td>
            </tr>
            <tr>
              <td>Cutscene Skip</td>
              <td>Skip cutscenes and avoid forced codecs.</td>
            </tr>
            <tr>
              <td>Dark Mode</td>
              <td>Hides the gameplay onscreen. If wanting to play with radar, the UnlockRadar module is also recommended</td>
            </tr>
            <tr>
              <td>Default Options</td>
              <td>Set the contents of the Options menu when the game launches</td>
            </tr>
            <tr>
              <td>Delayed Load</td>
              <td>Insert an artifical pause on every load (e.g. for practice)</td>
            </tr>
            <tr>
              <td>DirectInput Background</td>
              <td>Allow DirectInput controllers to be used when the game is not in focus</td>
            </tr>
            <tr>
              <td>Drebin Mode</td>
              <td>Unlock all weapons with 9999 ammo. Port of the V's Fix patch by WhiteSnoop.</td>
            </tr>
            <tr>
              <td>Equip Shortcuts</td>
              <td>Use keyboard/gamepad buttons to change weapons a la MGS1 PC</td>
            </tr>
            <tr>
              <td>Game Over</td>
              <td>Add "Game Over If Caution".</td>
            </tr>
            <tr>
              <td>New Game Info</td>
              <td>Display information about game settings on the New Game screen</td>
            </tr>
            <tr>
              <td>No Quit Prompt</td>
              <td>Stop the confirmation prompt from appearing when you attempt to close the game</td>
            </tr>
            <tr>
              <td>PS2 Controls</td>
              <td>Modify the game controls to more closely match the PS2 versions (e.g. swap X and O)</td>
            </tr>
            <tr>
              <td>Save Location Changer</td>
              <td>Change the directory that save files are stored, with multiple directories and hotswapping</td>
            </tr>
            <tr>
              <td>Save Menu Helpers</td>
              <td>Allow 100 save games, customisable save titles, and skip straight to the first/last save by pressing
                L2/R2</td>
            </tr>
            <tr>
              <td>Soft Reset</td>
              <td>Quit to the title screen when pressing L1+L2+R1+R2+Start</td>
            </tr>
            <tr>
              <td>Stats Display</td>
              <td>Show the current game stats when paused, or display selected live-updating stats while playing</td>
            </tr>
            <tr>
              <td>Style</td>
              <td>Add lost PS2-era features such as the musical ringtone and codename easter egg</td>
            </tr>
            <tr>
              <td>Text Change</td>
              <td>Change the text on many life gauges. Typically used to show the player's name.</td>
            </tr>
            <tr>
              <td>Timer Display</td>
              <td>Display the current game/real time, for the whole game and for individual areas.</td>
            </tr>
            <tr>
              <td>Turbo</td>
              <td>Add turbo capabilities to your input, as an alternative to XInputPlus turbo</td>
            </tr>
            <tr>
              <td>Turbo Display</td>
              <td>Show whether your turbo solution (ASI or XInputPlus) is enabled, and play custom SFX</td>
            </tr>
            <tr>
              <td>Unlock Radar</td>
              <td>Show the radar in most situations. Port of the V's Fix patch by WhiteSnoop.</td>
            </tr>
            <tr>
              <td>VR Randomiser</td>
              <td>Missions in the menu are shuffled</td>
            </tr>
          </tbody>
        </table>
        <div class="content-spacer"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Styles for the overall help page */
.help-page {
  height: 100%;
  overflow: hidden;
}

/* Styles for the main content container */
.help-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px 20px 40px;
  color: #e6e6e6;
  font-family: Arial, sans-serif;
  line-height: 1.6;
  height: 100%;
}

/* Spacer at the bottom of the content */
.content-spacer {
  height: 50px;
}

/* Heading styles */
h1, h2 {
  color: #ffffff;
  margin-top: 1.5em;
  margin-bottom: 0.5em;
}

h1 {
  text-align: center;
  font-size: 2em;
}

h2 {
  font-size: 1.5em;
}

/* Styles for paragraphs, lists, and code blocks */
p, ul, pre {
  margin-bottom: 1em;
}

ul {
  padding-left: 20px;
}

pre {
  background-color: rgba(101, 100, 100, 0.45);
  border: 1px solid #ddd;
  border-radius: 4px;
  padding: 10px;
  font-family: monospace;
  white-space: pre-wrap;
  word-wrap: break-word;
}

/* Horizontal rule style */
hr {
  border: 0;
  border-top: 1px solid #ddd;
  margin: 20px 0;
}

/* Link styles */
a {
  color: #4285f4;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}

/* Styles for the mod descriptions table */
.help-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 1em;
}

.help-table th,
.help-table td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}

.help-table th {
  background-color: rgba(66, 133, 244, 0.2);
  font-weight: bold;
}

.help-table tr:nth-child(even) {
  background-color: rgba(255, 255, 255, 0.05);
}

/* Responsive styles for smaller screens */
@media (max-width: 768px) {
  .help-container {
    padding: 10px;
  }

  h1 {
    font-size: 1.5em;
  }

  h2 {
    font-size: 1.2em;
  }

  .help-table th,
  .help-table td {
    padding: 5px;
  }
}
.help-page {
  height: 100%;
  overflow: visible;
}

.help-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 0px;
  color: #e6e6e6;
  font-family: Arial, sans-serif;
  line-height: 1.6;

  height: 100%;
}

/* Webkit-specific scrollbar styles */
.help-container::-webkit-scrollbar {
  width: 8px;
}

.help-container::-webkit-scrollbar-track {
  background: #2a2a2e;
}

.help-container::-webkit-scrollbar-thumb {
  background-color: #4285f4;
  border-radius: 4px;
}
</style>

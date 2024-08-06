import { library } from '@fortawesome/fontawesome-svg-core'

// Import specific icons from FontAwesome's free solid icons set
import {
  faDownload,   // Download icon
  faCode,       // Code icon
  faLock,       // Lock icon
  faCog,        // Cog/Settings icon
  faFileAlt,    // File icon
  faListAlt,    // List icon
  faGamepad,    // Gamepad icon
  faFloppyDisk, // Floppy disk/Save icon
  faDisplay,    // Display/Monitor icon
  faGaugeHigh,  // Gauge/Speedometer icon
  faQuestion,   // Question mark icon
  faSliders,    // Sliders/Adjustments icon
  faEarthAmericas // Globe icon showing North America
} from '@fortawesome/free-solid-svg-icons'

// Add the imported icons to the FontAwesome library
library.add(
  faDownload,
  faCode,
  faLock,
  faCog,
  faFileAlt,
  faListAlt,
  faGamepad,
  faFloppyDisk,
  faDisplay,
  faGaugeHigh,
  faQuestion,
  faSliders,
  faEarthAmericas
)

// Note: No need to export anything as the library is modified globally

<script>
  import { h } from 'vue'
  import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

  // Create a separator component
  const separator = h('hr', {
    style: {
      borderColor: 'rgba(0, 0, 0, 0.9)',
      margin: '10px',
    },
  })

  // Function to create FontAwesome icons
  const faIcon = (props) => {
    return {
      element: h('div', [h(FontAwesomeIcon, { size: 'lg', ...props })]),
    }
  }

  export default {
    name: 'Sidebar',
    emits: ['sidebar-toggle'],
    data() {
      return {
        // Sidebar menu structure
        menu: [
          // {
          //   header: 'Mods',
          //   hiddenOnCollapse: false,
          // },
          // {
          //   component: separator,
          // },
          {
            title: 'Global Settings',
            icon: faIcon({ icon: 'fa-solid fa-earth-americas' }),
            href: '/MGS2.ini',
          },
          {
            component: separator,
          },
          {
            title: 'HUD',
            icon: faIcon({ icon: 'fa-solid fa-display' }),
            child: [
              {
                href: '/MGS2.Timer.ini',
                title: 'Timer',
              },
              {
                href: '/MGS2.TextChange.ini',
                title: 'Text Change',
              },
              {
                href: '/MGS2.Stats.ini',
                title: 'Stats Display',
                disabled: false,
              },
              {
                href: '/MGS2.Performance.ini',
                title: 'FPS Counter',
                disabled: false,
              },
              {
                href: '/MGS2.Info.ini',
                title: 'Practice Info',
                disabled: false,
              },
              {
                href: '/MGS2.NewGameInfo.ini',
                title: 'New Game Info',
                disabled: false,
              },
            ]
          },
          {
            component: separator,
            hidden: true
          },
          {
            title: 'Turbo',
            icon: faIcon({ icon: 'fa-solid fa-gauge-high'}),
            child: [
              {
                href: '/MGS2.Turbo.ini',
                title: 'Input Config',
              },
              {
                href: '/MGS2.TurboDisplay.ini',
                title: 'Display Config',
              },
            ],
          },
          {
            component: separator,
            hidden: true
          },
          {
            title: 'Modifiers',
            icon: faIcon({ icon: 'fa-solid fa-sliders' }),
            disabled: false,
            child: [
              {
                href: '/MGS2.Caution.ini',
                title: 'Caution/Evasion %',
                hidden: false,
              },
              {
                href: '/MGS2.GameOver.ini',
                title: 'Game Over if Caution',
                hidden: false,
              },
              {
                href: '/MGS2.Wet.ini',
                title: 'Wet %',
                hidden: false,
              },
              {
                href: '/MGS2.FirstPerson.ini',
                title: 'First Person',
                hidden: false,
              },
              {
                href: '/MGS2.Ames.ini',
                title: 'Ames',
                hidden: false,
              },
              {
                href: '/MGS2.VRRando.ini',
                title: 'VR Randomizer',
                hidden: false,
              },
              {
                href: '/MGS2.DrebinMode.ini',
                title: 'Drebin Mode',
                hidden: false
              },
              {
                href: '/MGS2.UnlockRadar.ini',
                title: 'Unlock Radar',
                hidden: false,
              },
            ],
          },
          {
            component: separator,
            hidden: true
          },
          {
            title: 'Input & Shortcuts',
            icon: faIcon({ icon: 'fa-solid fa-gamepad' }),
            disabled: false,
            child: [
              {
                href: '/MGS2.DInputBackground.ini',
                title: 'DInput Background Support',
                hidden: false,
              },
              {
                href: '/MGS2.SoftReset.ini',
                title: 'Soft Reset',
                hidden: false,
              },
              {
                href: '/MGS2.Actions.ini',
                title: 'Actions',
                hidden: false,
              },
              {
                href: '/MGS2.EquipShortcuts.ini',
                title: 'Item & Weapon Shortcuts',
                hidden: false,
              },
              {
                href: '/MGS2.PS2Controls.ini',
                title: 'PS2 Controls',
                hidden: false,
              },
            ],
          },
          {
            component: separator,
            hidden: true
          },
          {
            title: 'Game Saves',
            icon: faIcon({ icon: 'fa-solid fa-floppy-disk'}),
            disabled: false,
            child: [
              {
                href: '/MGS2.SaveLocation.ini',
                title: 'Save Location',
                hidden: false,
              },
              {
                href: '/MGS2.SaveMenu.ini',
                title: 'Save Menu',
                hidden: false,
              },
            ],
          },
          {
            title: 'Miscellaneous',
            icon: faIcon({ icon: 'fa-solid fa-list-alt' }),
            hidden: false,
            disabled: false,
            child: [
            {
                href: '/MGS2.Options.ini',
                title: 'Game Options',
                hidden: false,
              },
              {
                href: '/MGS2.NoQuitPrompt.ini',
                title: 'No Quit Prompt',
                hidden: false,
              },
              {
                href: '/MGS2.CutsceneSkip.ini',
                title: 'Cutscene Skip',
                hidden: false,
              },
              {
                href: '/MGS2.DelayedLoad.ini',
                title: 'Delayed Load',
                hidden: false,
              },
              {
                href: '/MGS2.Affinity.ini',
                title: 'Affinity',
                hidden: false,
              },
            ]
          },
          {
            component: separator,
            hidden: false
          },
          {
            title: 'Help',
            href: '/Help',
            icon: faIcon({ icon: 'fa-solid fa-question'}),
          },
          {
            title: "Bmn's ASI Mod",
            href: 'https://mgs.w00ty.com/mgs2/asi/',
            external: true,
            icon: faIcon({ icon: 'fa-solid fa-code'}),
          },
        ],
        collapsed: false,
        relative: true,
        width: '250px',
        themes: [
          {
            name: 'Default theme',
            input: '',
          },
          {
            name: 'White theme',
            input: 'white-theme',
          },
        ],
        selectedTheme: '',
        isOnMobile: false,
      }
    },
    mounted() {
      // Set initial state and add resize listener
      this.onResize()
      window.addEventListener('resize', this.onResize)
    },
    methods: {
      // Handle sidebar collapse toggle
      onToggleCollapse(collapsed) {
        console.log('onToggleCollapse', collapsed)
        this.collapsed = collapsed
        this.$emit('sidebar-toggle', collapsed)
      },
      // Handle menu item click
      onItemClick(event, item) {
        console.log('onItemClick', item)
      },
      // Handle window resize
      onResize() {
        if (window.innerWidth <= 768) {
          this.isOnMobile = true
          this.collapsed = true
        } else {
          this.isOnMobile = false
          this.collapsed = false
        }
        this.$emit('sidebar-toggle', this.collapsed)
      },
      // Change sidebar theme
      changeTheme(theme) {
        this.selectedTheme = theme
      },
    },
  }
</script>

<template>
  <!-- Sidebar menu component -->
  <sidebar-menu
    v-model:collapsed="collapsed"
    :menu="menu"
    :theme="selectedTheme"
    :show-one-child="true"
    :width="width"
    :relative="relative"
    @update:collapsed="onToggleCollapse"
    @item-click="onItemClick"
  />
</template>

<style lang="scss">
  // Custom variables for sidebar styling
  $primary-color: #939aa153;
  $base-bg: #1a68b58a;
  $item-color: #fff;
  $item-hover-color: #0f0f0f;
  $item-hover-bg: #6a6a6a;
  $item-open-color: #000000;
  $item-open-bg: #4d4d65;
  $item-active-color: #000000;
  //$item-active-bg:;
  //$item-active-line-color: #2a2a2e;
  $item-font-size: 16px;
  $item-line-height: 45px;
  $item-padding: 10px 15px;
  //$dropdown-bg: #36363b;
  $header-item-color: rgba(255, 255, 255, 0.7);
  $toggle-btn-color: #FFF;
  $toggle-btn-bg: rgba(30, 30, 30, 0.303);
  $icon-height: 35px;
  $icon-width: 35px;
  //$icon-color: #fff;
  //$icon-bg: #1e1e21;
  //$icon-active-color:;
  //$icon-active-bg:;
  //$icon-open-color:;
  //$icon-open-bg:;

  // Import sidebar menu styles
  @import "vue-sidebar-menu/src/scss/vue-sidebar-menu.scss";
  @import url('https://fonts.googleapis.com/css?family=Source+Sans+Pro:400,600');

  body {
    margin: 0;
    padding: 0;
    font-family: 'Source Sans Pro', sans-serif;
    font-size: 18px;
    background-color: #2f2f2fcd;
    color: #f7fbff;
  }

  .container {
    width: auto;
  }

  // Sidebar menu styles
  .v-sidebar-menu {
    &.vsm_collapsed {
      width: auto;
    }
  }
</style>

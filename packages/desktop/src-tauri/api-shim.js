;(function () {
  const invoke = window.__TAURI__.core.invoke
  const event = window.__TAURI__.event

  const wslServers = {
    getState: () => invoke('wsl_get_state'),
    subscribe: (cb) => event.listen('wsl-event', (e) => cb(e.payload)),
    probeRuntime: () => invoke('wsl_probe_runtime'),
    refreshDistros: () => invoke('wsl_refresh_distros'),
    installWsl: () => invoke('wsl_install_wsl'),
    installDistro: (n) => invoke('wsl_install_distro', { name: n }),
    probeAddable: (d) => invoke('wsl_probe_addable', { distros: d }),
    installOpencode: (n) => invoke('wsl_install_opencode', { name: n }),
    openTerminal: (n) => invoke('wsl_open_terminal', { name: n }),
    addServer: (d) => invoke('wsl_add_server', { distro: d }),
    removeServer: (id) => invoke('wsl_remove_server', { id }),
    startServer: (id) => invoke('wsl_start_server', { id }),
  }

  let updaterUnsub = null
  let updaterState = null
  const updaterCbs = new Set()
  const updaterHandler = (e) => { updaterState = e.payload; updaterCbs.forEach((cb) => cb(e.payload)) }

  const updater = {
    subscribe: async (cb) => {
      updaterCbs.add(cb)
      if (updaterState) cb(updaterState)
      if (!updaterUnsub) updaterUnsub = await event.listen('updater-state', updaterHandler)
      return () => { updaterCbs.delete(cb); if (updaterCbs.size === 0) { updaterUnsub(); updaterUnsub = null } }
    },
    check: () => invoke('updater_check'),
    install: () => invoke('updater_install'),
  }

  window.api = {
    killSidecar: () => invoke('kill_sidecar'),
    installCli: () => invoke('install_cli'),
    awaitInitialization: () => invoke('await_initialization'),
    wslServers,
    updater,
    consumeInitialDeepLinks: () => invoke('consume_initial_deep_links'),
    getDefaultServerUrl: () => invoke('get_default_server_url'),
    setDefaultServerUrl: (url) => invoke('set_default_server_url', { url }),
    isFirstLaunchOnboardingPending: () => invoke('is_first_launch_onboarding_pending'),
    finishFirstLaunchOnboarding: (c) => invoke('finish_first_launch_onboarding', { createDefaultProject: c }),
    getDisplayBackend: () => invoke('get_display_backend'),
    setDisplayBackend: (b) => invoke('set_display_backend', { backend: b }),
    parseMarkdownCommand: (m) => invoke('parse_markdown', { markdown: m }),
    checkAppExists: (n) => invoke('check_app_exists', { appName: n }),
    resolveAppPath: (n) => invoke('resolve_app_path', { appName: n }),
    storeGet: (n, k) => invoke('store_get', { name: n, key: k }),
    storeSet: (n, k, v) => invoke('store_set', { name: n, key: k, value: v }),
    storeDelete: (n, k) => invoke('store_delete', { name: n, key: k }),
    storeClear: (n) => invoke('store_clear', { name: n }),
    storeKeys: (n) => invoke('store_keys', { name: n }),
    storeLength: (n) => invoke('store_length', { name: n }),
    getWindowCount: () => invoke('get_window_count'),
    getWindowID: () => invoke('get_window_id'),
    onMenuCommand: (cb) => event.listen('menu-command', (e) => cb(e.payload)),
    onDeepLink: (cb) => event.listen('deep-link', (e) => cb(e.payload)),
    openDirectoryPicker: (o) => invoke('open_directory_picker', { opts: o }),
    openFilePicker: (o) => invoke('open_file_picker', { opts: o }),
    readPickedFile: (t, p) => invoke('read_picked_file', { token: t, path: p }),
    releasePickedFiles: (t) => invoke('release_picked_files', { token: t }),
    getPathForFile: () => null,
    saveFilePicker: (o) => invoke('save_file_picker', { opts: o }),
    openLink: (url) => invoke('open_link', { url }),
    openPath: (p, a) => invoke('open_path', { path: p, app: a }),
    readClipboardImage: () => invoke('read_clipboard_image'),
    showNotification: (t, b) => invoke('show_notification', { title: t, body: b }),
    getWindowFocused: () => invoke('get_window_focused'),
    setWindowFocus: () => invoke('set_window_focus'),
    showWindow: () => invoke('show_window'),
    relaunch: () => invoke('relaunch'),
    getZoomFactor: () => invoke('get_zoom_factor'),
    setZoomFactor: (f) => invoke('set_zoom_factor', { factor: f }),
    getPinchZoomEnabled: () => invoke('get_pinch_zoom_enabled'),
    setPinchZoomEnabled: (e) => invoke('set_pinch_zoom_enabled', { enabled: e }),
    onPinchZoomEnabledChanged: (cb) => event.listen('pinch-zoom-enabled-changed', (e) => cb(e.payload)),
    onZoomFactorChanged: (cb) => event.listen('zoom-factor-changed', (e) => cb(e.payload)),
    setTitlebar: (t) => invoke('set_titlebar', { theme: t }),
    runDesktopMenuAction: (a) => invoke('run_desktop_menu_action', { action: a }),
    setBackgroundColor: (c) => invoke('set_background_color', { color: c }),
    exportDebugLogs: () => invoke('export_debug_logs'),
    recordFatalRendererError: (e) => invoke('record_fatal_renderer_error', { error: e }),
  }
})()

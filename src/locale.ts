import { createI18n } from "vue-i18n";

// a mapping between locale abbreviations and full Language names
export const languageNames = {
  de: "🇩🇪 Deutsch",
  en: "🇬🇧 English",
  fr: "🇫🇷 Français",
  it: "🇮🇹 Italiano",
};

const messages = {
  // =========================================================================
  // 🇬🇧 ENGLISH
  // =========================================================================

  en: {
    setup: {
      setup: {
        welcome: "Holochain Launcher",
        slogan: "Discover, install and easily manage your Holochain apps",
      },
      login: {
        enterPassword: "Enter password",
        forgotPassword: "forgot password",
        invalidPassword: "Invalid Password.",
        slogan: "Discover, install and easily manage your Holochain apps",
      },
      changeLanguage: "change language",
      settingUp: "Setting up",
      startingUp: "Starting up",
    },
    main: {
      activateDevMode: "Enable Developer Mode (for App Developers)",
      activePeerSynchronization: "Active Peer Synchronization",
      allVersions: "All Versions",
      copied: "Copied",
      copyDnaHash: "Copy Dna Hash",
      copyNetworkSeed: "Copy Network Seed",
      cloneName: "Name",
      developerMode: "Developer Mode",
      devModeExplainer:
        "If you are an app developer and want to publish apps, activate the developer mode.",
      happVersion: "happ Version",
      hide: "Hide",
      hideNetworkSeed: "Hide Network Seed",
      holochainVersion: "Holochain Version",
      incoming: "incoming",
      installNewApp: "INSTALL NEW APP",
      inThisVersion: "in this Holochain Version.",
      launcher: "Launcher",
      name: "name",
      nameDescending: "name descending",
      networkSeed: "Network Seed",
      noApps: "There are no apps installed",
      noClonedCells: "There are no cloned cells in this app.",
      noDisabledClonedCells: "There are no disabled cloned cells in this app.",
      outgoing: "outgoing",
      refresh: "Refresh",
      reportIssue: "Report Issue",
      settings: "Settings",
      sortBy: "sort by",
      show: "Show",
      showNetworkSeed: "Show Network Seed",
      uiVersion: "UI Version",
      unknown: "unknown",
      updatesAvailable: "(Updates Available)",
      visiblePeers: "Visible Peers",
      yourPublicKey: "Your Public Key",
    },
    appStore: {
      activeDataExchanges: "currently ongoing data exchanges with peers",
      appStore: "App Store",
      appLibrarySynchronization: "App Library Synchronization",
      fullSynchronizationRequired:
        "Full synchronization with peers required to reliably download all apps.",
      inQueue: "in queue",
      noOngoingPeerSynchronization: "no ongoing peer synchronization",
      selectAppFromFileSystem: "Select app from filesystem",
      noAppsInStore:
        "There are no apps available yet in the App Library or Synchronization with Peers is not complete.",
      noAppsForSearch: "No apps found for search input...",
      receivingData: "Receiving new data",
      searchingForPeers: "Searching for peers",
      searchingForPeersDetail:
        "Looking for other peers in the App Store network to fetch the list of installable apps.",
      synchronizationNotCompleteError:
        "App Library Synchronization not Complete. Please try again later.",
    },
    launcher: {
      filesystem: "Filesystem",
      getStarted: "To get started, install your first app:",
    },
    settings: {
      advancedSettings: "Advanced Settings",
      appSettings: "App Settings",
      appSettingsHelper:
        "Settings for Holochain Apps with Graphical User Interfaces",
      general: "General",
      headlessApps: "Headless Apps",
      headlessAppsHelper: "Holochain Apps without Graphical User Interface",
      holochainVersions: "Holochain Versions",
      holochainVersionsHelper: "Installed Holochain Versions",
      launcherConfiguration: "Launcher Configuration",
      launcherConfigurationDescription:
        "Adjust log level, bootstrap server, signaling server, or use a custom holochain binary",
      noHeadlessApps: "There are no headless apps installed",
      noHolochainVersions: "There are no Holochain versions installed.",
      noWebApps: "There are no web apps installed",
      openAppStore: "Open App Store",
      openDevHub: "Open Dev Hub",
      publicKey: "Public Key",
      publishAnApp: "Publish an app",
    },
    buttons: {
      back: "Back",
      cancel: "Cancel",
      close: "Close",
      continue: "continue",
      disable: "Disable",
      enable: "Enable",
      getStarted: "Get started",
      install: "Install",
      letsGo: "Let's Go!",
      next: "Next",
      no: "No",
      ok: "Ok",
      quit: "Quit",
      restart: "Restart",
      retry: "Retry",
      save: "Save",
      start: "Start",
      uninstall: "Uninstall",
      uninstalling: "uninstalling...",
      update: "Update",
      yes: "Yes",
    },
    dialogs: {
      config: {
        restoreDefaults: "Reset to Default Values",
        saveAndRestart: "Save and Restart",
      },
      restart:
        "Do you want to clean up all holochain processes and restart the Holochain Launcher?",
      setupPassword: {
        title: "Set up password",
        part1:
          "To get started, you need to set up a password for the Holochain Launcher.",
        part2:
          "This password is used to secure your cryptographic private keys associated with your identities in Holochain apps.",
        choosePassword: "Choose your password",
        inputPlaceholder: "Choose password",
        repeatPassword: "Repeat your password",
        inputPlaceholder2: "Repeat password",
        passwordsDontMatch: "Passwords don't match.",
        warningPart1: "This password ",
        warningBold1: "cannot be reset or changed ",
        warningPart2:
          "without losing access to your Holochain apps on this device.",
        warningPart3:
          "Make sure to write it on paper right now and create at least one backup of it in a safe place.",
        confirmation:
          "I confirm that I have created a safe backup of my password.",
      },
      forgotPassword: {
        title: "Oh my!",
        part1:
          "Unfortunately, the Holochain Launcher does not support a password recovery mechanism at this stage.",
        part2:
          'Your only option is to do a factory reset (Settings > Factory Reset) to delete your current profile alongside with all your holochain apps and restart from scratch. You will need to select the option "Delete data of ALL Holochain versions, reset password and delete private keys".',
      },
      factoryReset: {
        title: "Factory Reset",
        part1: "This will ",
        bold1: "uninstall all Holochain apps ",
        part2:
          "on your computer of Holochain versions supported by this version of the Holochain Launcher.",
        part3: "It will also ",
        bold2: "delete all data stored in those apps.",
        primaryButton: "Execute Factory Reset",
        executing: "Executing...",
        optionalDeletions: "Optional:",
        deleteLair: "Delete Private Keys",
        deleteLogs: "Delete Logs",
        deleteAllHolochainVersions:
          "(not recommended) Delete data of ALL Holochain versions, reset password and delete private keys. This can include deletion of Holochain versions that are or were used by other versions of the Holochain Launcher.",
      },
      guiUpdate: {
        title: "UI Update",
        mainText: "There's a new UI available for this app",
        changelog: "changelog",
        version: "version",
        publishedAt: "published",
        question: "Would you like to install it?",
      },
      changeLanguage: {
        language: "Language",
        languageSettings: "Language Settings",
      },
      confirmUninstallApp:
        "Are you sure you want to uninstall this App? This will irrevocably delete all data stored in it.",
      confirmUninstallDevHub: {
        text: "Uninstalling the DevHub means that you permanently lose editor access to any apps that you have published with this instance of the DevHub (i.e. with the associated public key). This means that you won't be able to publish any further releases or updates to those apps - even if you later reinstall the DevHub.",
        confirmation:
          "I have read the warning and confirm that I want to uninstall the DevHub.",
      },
      confirmUninstallCell:
        "Are you sure you want to delete this cell? This will irrevocably delete all data stored in it.",
      networkStats: {
        networkStats: "Network Statistics",
        changeHolochainVersion: "Choose Holochain Version",
      },
      warning: "Warning",
    },
  },

  // =========================================================================
  // 🇩🇪 GERMAN
  // =========================================================================

  de: {
    setup: {
      setup: {
        welcome: "Holochain Launcher",
        slogan: "Entdecke, installiere und verwalte deine Holochain Apps",
      },
      login: {
        enterPassword: "Passwort",
        forgotPassword: "Passwort vergessen",
        invalidPassword: "Ungültiges Passwort.",
        slogan: "Entdecke, installiere und verwalte deine Holochain Apps",
      },
      changeLanguage: "Sprache ändern",
      startingUp: "Starten",
      settingUp: "Einrichten",
    },
    main: {
      activateDevMode: "Entwickler-Modus aktivieren (für App-Entwickler)",
      activePeerSynchronization: "Aktive Peer Synchronisationen",
      allVersions: "Alle Versionen",
      copied: "Kopiert",
      copyDnaHash: "Dna Hash kopieren",
      copyNetworkSeed: "Network Seed kopieren",
      cloneName: "Name",
      developerMode: "Entwickler Modus",
      devModeExplainer:
        "Wenn du ein App-Entwickler bist und Apps publizieren möchtest, kannst du den Entwickler-Modus aktivieren.",
      happVersion: "happ Version",
      hide: "Verbergen",
      hideNetworkSeed: "Network Seed verbergen",
      holochainVersion: "Holochain Version",
      incoming: "eingehend",
      installedApps: "Installierte Apps",
      installNewApp: "APP INSTALLIEREN",
      inThisVersion: "in dieser Holochain Version.",
      name: "Name",
      nameDescending: "Name absteigend",
      networkSeed: "Network Seed",
      noClonedCells: "Diese App hat keine Cloned Cells.",
      noDisabledClonedCells: "Diese App hat keine deaktivierten Cloned Cells.",
      noApps: "Es sind keine Apps installiert.",
      outgoing: "ausgehend",
      refresh: "Aktualisieren",
      reportIssue: "Problem Melden",
      settings: "Einstellungen",
      show: "Zeigen",
      showNetworkSeed: "Network Seed anzeigen",
      sortBy: "sortiere nach",
      uiVersion: "UI Version",
      unknown: "unbekannt",
      updatesAvailable: "(Updates verfügbar)",
      visiblePeers: "Sichtbare Peers",
      yourPublicKey: "Dein Public Key",
    },
    appStore: {
      activeDataExchanges: "Zurzeit aktiver Datenaustausch mit Peers",
      appStore: "App Store",
      appLibrarySynchronization: "App Store Synchronisation",
      fullSynchronizationRequired:
        "Vollständige Synchronisation mit Peers erforderlich, um alle Apps herunterladen zu können.",
      inQueue: "ausstehend",
      noOngoingPeerSynchronization: "keine aktiven Synchronisationen mit Peers",
      selectAppFromFileSystem: "Installiere App von Datei",
      noAppsInStore:
        "Es sind noch keine Apps im App Store verfügbar oder die Synchronisierung mit Peers ist noch unvollständig.",
      noAppsForSearch: "Keine Apps gefunden für die gewählte Such-Eingabe...",
      receivingData: "Neue Daten werden empfangen",
      searchingForPeers: "Suche Peers",
      searchingForPeersDetail:
        "Suche nach anderen Peers im App Store Netzwerk um die Liste installierbarer Apps zu erhalten.",
      synchronizationNotCompleteError:
        "App Store Synchronisation noch nicht abgeschlossen. Bitte versuche es später erneut.",
    },
    launcher: {
      filesystem: "Aus Datei",
      getStarted: "Leg los und installiere deine erste App:",
    },
    settings: {
      advancedSettings: "Erweiterte Einstellungen",
      appSettings: "App Settings",
      appSettingsHelper:
        "Settings for Holochain Apps mit Graphischer Benutzeroberfläche",
      headlessApps: "Headless Apps",
      headlessAppsHelper: "Holochain Apps ohne Graphische Benutzeroberfläche",
      holochainVersions: "Holochain Versionen",
      holochainVersionsHelper: "Installierte Holochain Versionen",
      launcherConfiguration: "Launcher Konfiguration",
      launcherConfigurationDescription:
        "Passen Sie die Protokollebene, den Bootstrap-Server, den Proxy-Server an oder verwenden Sie eine benutzerdefinierte Holochain-Binärdatei",
      noHeadlessApps: "Es sind keine Headless Apps installiert",
      noHolochainVersions: "Es sind keine Holochain Versionen installiert.",
      noWebApps: "Es sind keine Web Apps installiert",
      openAppStore: "Open App Store",
      openDevHub: "Open Dev Hub",
      publicKey: "Public Key",
      publishAnApp: "Eigene App Publizieren",
    },
    buttons: {
      back: "Zurück",
      cancel: "Abbrechen",
      close: "Schliessen",
      continue: "Weiter",
      disable: "Deaktivieren",
      enable: "Aktivieren",
      getStarted: "Loslegen",
      install: "Installieren",
      letsGo: "Let's Go!",
      next: "Weiter",
      no: "Nein",
      ok: "Ok",
      restart: "Neustart",
      retry: "Erneut versuchen",
      save: "Speichern",
      start: "Starten",
      uninstall: "Deinstallieren",
      uninstalling: "deinstallieren...",
      update: "Update",
      quit: "Beenden",
      yes: "Ja",
    },
    dialogs: {
      config: {
        restoreDefaults: "Zurücksetzen zu Standardwerten",
        saveAndRestart: "Speichern und Neustart",
      },
      restart:
        "Möchtest du alle Holochain Prozesse beenden und den Holochain Launcher neu starten?",
      setupPassword: {
        title: "Passwort erstellen",
        part1:
          "Um loszulegen brauchst du ein Passwort für den Holochain Launcher.",
        part2:
          "Das Passwort wird verwendet, um deine kryptographischen Schlüssel zu schützen, die mit deinen Identitäten in Holochain Apps assoziiert sind.",
        choosePassword: "Wähle dein Passwort",
        inputPlaceholder: "Passwort eingeben",
        repeatPassword: "Wiederhole dein Passwort",
        inputPlaceholder2: "Passwort eingeben",
        passwordsDontMatch: "Passwörter stimmen nicht überein.",
        warningPart1: "Dieses Passwort ",
        warningBold1: "kann nicht zurückgesetzt oder geändert werden ",
        warningPart2:
          "ohne Zugrif zu deinen Holochain Apps auf diesem Gerät zu verlieren.",
        warningPart3:
          "Notiere es jetzt direkt auf Papier und erstelle mindestens ein sicheres Backup an einem sicheren Ort.",
        confirmation:
          "I bestätige, dass ich ein sicheres Backup meines Passworts gemacht habe.",
      },
      forgotPassword: {
        title: "Oh nein!",
        part1:
          "Leider unterstützt Holochain Launcher momentan noch keinen Passwort Wiederherstellungs-Mechanismus.",
        part2:
          'Die einzige Option ist, den Holochain Launcher komplett zurückzusetzen (Settings > Factory Reset) um dein Profil und all deine Holochain Apps zu löschen und von vorne zu beginnen. Du musst dabei die Option "Daten ALLER Holochain Versionen löschen, Passwort zurücksetzen und Private Keys löschen" wählen.',
      },
      factoryReset: {
        title: "Zurücksetzen",
        part1: "Dies ",
        bold1: "deinstalliert all deine Holochain Apps ",
        part2:
          "auf deinem Computer von Holochain Versionen die von dieser Version des Holochain Launchers unterstützt werden.",
        part3: "Dabei werden auch ",
        bold2: "alle Daten gelöscht, die in diesen Apps gespeichert sind.",
        primaryButton: "Zurücksetzen",
        executing: "Ausführen...",
        optionalDeletions: "Optional:",
        deleteLair: "Private Schlüssel löschen",
        deleteLogs: "Log Dateien löschen",
        deleteAllHolochainVersions:
          "(Nicht empfohlen) Daten ALLER Holochain Versionen löschen, Passwort zurücksetzen und Private Keys löschen. Dies kann das Löschen von Holochain Versionen beinhalten, die von anderen Versionen des Holochain Launcher verwendet werden oder wurden.",
      },
      guiUpdate: {
        title: "UI Update",
        mainText: "Es ist ein neues UI verfügbar für diese App",
        changelog: "Änderungen",
        version: "Version",
        publishedAt: "Publiziert",
        question: "Möchtest du es installieren?",
      },
      changeLanguage: {
        language: "Sprache",
        languageSettings: "Sprach-Einstellungen",
      },
      confirmUninstallApp:
        "Bist du sicher, dass du diese App deinstallieren möchtest? Dies wird unwiderruflich alle Daten löschen, die darin gespeichert sind.",
      confirmUninstallDevHub: {
        text: "Wenn du den DevHub deinstallierst verlierst du unwiderruflich die Bearbeitungsrechte für jegliche Apps die du mit dieser DevHub-Instanz (sprich mit dem dazugehörigen Public Key) publiziert hast. Dies bedeutet, dass du keine weiteren Releases oder Updates für diese Apps mehr publizieren kannst - auch dann, wenn du den DevHub wieder neu installierst.",
        confirmation:
          "Ich habe die Warning gelesen und bestätige, dass ich den DevHub deinstallieren möchte.",
      },
      confirmUninstallCell:
        "Bist du sicher, dass du diese Cell deinstallieren möchtest? Dies wird unwiderruflich alle Daten löschen, die darin gespeichert sind.",
      networkStats: {
        networkStats: "Netzwerkstatistik",
        changeHolochainVersion: "Holochain Version",
      },
      warning: "Achtung",
    },
  },
};

export const i18n = createI18n({
  locale: "en",
  fallbackLocale: "en",
  messages,
});

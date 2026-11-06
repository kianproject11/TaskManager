
#define MyAppName "Task Manager"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Kian Project"
#define MyAppExeName "taskmanager.exe"
#define MyAppIcon "taskmanager_minimal.ico"

[Setup]
AppId={{A83D92B1-5E47-4A92-9B36-7C2F1D8E604A}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\Task Manager
DefaultGroupName={#MyAppName}
UninstallDisplayIcon={app}\{#MyAppExeName}
SetupIconFile=F:\projects\taskmanager\taskmanager_minimal.ico
PrivilegesRequired=admin
OutputDir=F:\projects\taskmanager
OutputBaseFilename=TaskManager_Setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional icons:"; Flags: unchecked

[Files]
Source: "F:\projects\taskmanager\target\release\taskmanager.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "F:\projects\taskmanager\taskmanager_minimal.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Task Manager"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\{#MyAppIcon}"
Name: "{autodesktop}\Task Manager"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\{#MyAppIcon}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Launch Task Manager"; Flags: postinstall nowait skipifsilent

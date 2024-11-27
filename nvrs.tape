# VHS
# https://github.com/charmbracelet/vhs

Output nvrs.gif

Require echo

Set Shell "zsh"
Set FontSize 32
Set Width 1280
Set Height 720
Set TypingSpeed 0ms
Set Theme { "black": "#000D19", "red": "#7D0C0C", "green": "#0C7D45", "yellow": "#7D750C", "blue": "#0C567E", "magenta": "#6E1D57", "cyan": "#12748F", "white": "#A3A3A3", "brightBlack": "#000D19", "brightRed": "#951313", "brightGreen": "#16A85F", "brightYellow": "#ABA119", "brightBlue": "#1A76A8", "brightMagenta": "#A01576", "brightCyan": "#188AA9", "brightWhite": "#A3A3A3", "background": "#00050D", "foreground": "#A3A3A3", "selection": "#6E1D57", "cursor": "#A3A3A3" }

Hide
Type "source /usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.plugin.zsh"
Enter
Type "source /usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.plugin.zsh"
Enter
Ctrl+L
Sleep 100ms
Show

Set TypingSpeed 30ms

Type "1. fetch latest versions of defined packages"
Sleep 1s
Ctrl+U
Sleep 500ms
Type "nvrs"
Sleep 500ms
Enter

Sleep 1.5s
Ctrl+L

Type "2. compare them to latest known versions"
Sleep 1s
Ctrl+U
Sleep 500ms
Type "nvrs --cmp"
Sleep 500ms
Enter

Sleep 1.5s
Ctrl+L

Type "3. update the known versions"
Sleep 1s
Ctrl+U
Sleep 500ms
Type "nvrs --take ALL"
Sleep 500ms
Enter

Sleep 2s

Enter
Type "4. go on github.com/adamperkowski/nvrs & leave a star"

Sleep 4s

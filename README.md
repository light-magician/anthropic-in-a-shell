# anthropic-in-a-shell
An aesthetic way to use Anthropic claude api via the command line.

## API Key Setup

### Getting Your API Key
1. You'll need a Claude API key from Anthropic to use this CLI tool
2. Visit https://console.anthropic.com/ to obtain your API key
3. Your API key should start with `sk-ant-`

### Setting Your API Key

Set your API key using the `setkey` command:
```bash
claude-cli setkey your-api-key-here
```

If you're running directly through cargo:
```bash
cargo run -- setkey your-api-key-here
```

Example:
```bash
claude-cli setkey sk-ant-api03xxxxxxxxxxxxx
```

### API Key Storage Location

Your API key is stored securely in a configuration file:

- **macOS**: `~/Library/Application Support/claude-cli/config.json`
- **Linux**: `~/.config/claude-cli/config.json`
- **Windows**: `C:\Users\<USERNAME>\AppData\Roaming\claude-cli\config.json`

To verify your API key was saved correctly:

**macOS**:
```bash
cat ~/Library/Application\ Support/claude-cli/config.json
```

**Linux**:
```bash
cat ~/.config/claude-cli/config.json
```

**Windows** (PowerShell):
```powershell
type $env:APPDATA\claude-cli\config.json
```

### Updating Your API Key

To update your API key, simply run the `setkey` command again with your new key:
```bash
claude-cli setkey your-new-api-key
```

### Security Note

The config file containing your API key is stored with user-only read/write permissions. However, please ensure:
- Keep your API key confidential
- Don't share your config file or API key
- If you suspect your API key has been compromised, regenerate it in the Anthropic console immediately

### Troubleshooting

If you get an error about missing API key:
1. Verify the key was saved correctly by checking the config file location above
2. Ensure the API key starts with `sk-ant-`
3. Try setting the key again using the `setkey` command
4. Check file permissions on the config directory and file
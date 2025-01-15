# Setup Arduino IDE for Teensy boards

## Install Arduino IDE v2

```
flatpak install flathub cc.arduino.IDE2
```

Run `Arduino IDE v2` via your app launcher, or do so in the terminal:

```
flatpak run cc.arduino.IDE2
```

### Setup Arduino IDE for Teensy 4.1

 * upstream docs: https://www.pjrc.com/teensy/td_download.html
 * Go to `File` -> `Preferences`
 * Scroll to the bottom to find `Additional boards manager URLS`.
 * Enter the Teensyduino repository URL: `https://www.pjrc.com/teensy/package_teensy_index.json`
 * While you're here, you can choose a nicer (darker) theme.
 * Click `OK`.
 * Go to `Tools` -> `Board` -> `Boards Manager`.
 * Search for  "teensy"" and install `Teensy (for Arduino IDE ...)` (latest version)

### Create new sketch

 * `File` -> `New Sketch`
 * In the `Select Board` dropdown, select `Teensy 4.1`.
 * Enter the example code:
 
```
// Simple LED blink

const int led = LED_BUILTIN;

void setup() {
  pinMode(led, OUTPUT);
}

void loop() {
  digitalWrite(led, HIGH);
  delay(100);
  digitalWrite(led, LOW);
  delay(100);
} 
```

 * Press the `Upload` (arrow point right -->) button.
   * The program should compile and upload to the Teensy device.
 * If successful, the program should quickly blink the (other) LED on
   the board.
 

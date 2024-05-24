# pastehaste

AutoHotKey Paste utility for speedy copy / pasting

## Copy / Pasting

- Input some text into the inputs for key numbers `1` - `5`

- Upon pressing `ALT + {num}` it will paste the text entered

- To hot copy into a slot, press `CTRL + ALT + {num}`

## Account lookup

- `ALT + W` will search for an email within the active process's title, and paste it into the current cursor selection

- `ALT + E` will paste a matching password from a lookup described below

  - Active process's title will be similarly searched for an email

  - `accounts.txt` in the local script dir has an expected format of the following

    ```text
    me@gmail.com,password
    me@gmail.com,password
    me@gmail.com,password
    me@gmail.com,password
    me@gmail.com,password
    ```

  - If lookup succeeds, the matching email's password will be pasted into the current cursor selection

## Saving

- Upon closing the program, the copied inputs will be saved to `key_inputs.txt` (in the script dir)

- Upon opening the program, if `key_inputs.txt` exists, it will load the values into the inputs

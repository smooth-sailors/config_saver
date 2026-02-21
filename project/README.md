## How Does the Microservice Work?

1. You must first have a TXT, CSV (only 2 columns, it might work with 3 but I haven't tested it), or MD file that holds a key-value list. Then you must trigger the executable in w/e language you're working in.
2. Once you do that, a file dialog will pop up that will want you to select the file holding your key-value list. The program will then spawn a terminal asking for what separates your keys and values. Input the character or string that acts as the separator.
3. Next, another file dialog will pop up. This one is to save the created JSON file that was made from your key-value list - feel free to drop the file anywhere. The JSON file is your new configuration file - keys and values will always be strings.
4. To load the JSON file, you'll need to figure out how to do that with w/e language you're working in. But because it's a JSON file, the process should be rather simple and possibly built into your language's standard library.
5. The important thing to note is that even if your keys and values were anything but strings before, then they'll now be strings. You'll have to reconvert the values back yourself in your language.

---

| | |
|---|---|
| **REQUEST** | Call/run the executable that matches with your operating system. |
| **RECEIVE** | Read and parse - if necessary - the JSON file that you made using the microservice. |

---

### Pros
- Configurations can be nigh-universally used since they're in JSON format.
- You can convert CSV, TXT, and MD files to JSON.
- The conversion process is rather simple.

### Cons
- You must save your key-value pairs to a TXT, CSV, or MD file first.
- Everything is in string format when saved.
- Loading will require you to parse the contents.

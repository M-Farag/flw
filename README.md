### FLW
Process text using configurable tasks.
The CLI app can read tasks from any .yml file that follow the task schema.

### Text Task Schema
```yaml
tasks:
  - data:
      - replace # command to replace text
      - original # text to replace
      - replace text # text to replace with
  - data:
      - count # command to count a word
      - word # word to count
```

### CSV Task Schema
```yaml 
tasks:
  - data:
    - replace # command to replace text
    - column_name # column to replace text in
    - original # text to replace
    - replace text # text to replace with
  - data:
    - replace # command to replace text
    - column_name # column to replace text in
    - original # text to replace
    - replace text # text to replace with
```

### Available Commands [WIP: more to come]
- Replace # on txt and csv
- Count # on txt only


### Usage
```bash
flw -f <path to the tasks .yml file> -i <path to input file> -t <type of file: txt or csv>

### Example for TXT
flw -f tasks.yml -i input.txt -t txt

### Example for CSV
flw -f tasks.yml -i input.csv -t csv
```

### Technical ToDos
- [ ] Add tests for the Runner module(s)
- [ ] Enhance the CSV Task processing for better performance
- [ ] Enhance the Error handling for better error messages


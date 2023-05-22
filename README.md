### FLW
Process text using configurable tasks.
The CLI app can read tasks from any .yml file that follow the task schema.

### Task Schema
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

### Available Commands [WIP: more to come]
- replace
- count


### Usage
```bash
flw -f <path to .yml file> -i <path to input file>
```

### Next Steps & Ideas
- [ ] [WIP] Add support for replacing data within a CSV file with a good level of configurability (e.g. replace data in a specific column(s))

### Technical todos
- [ ] Add two types of the Runner module: one for txt files and one for csv files
- [ ] Add tests for the Runner module(s)
- [ ] Add a trait for the Runner module(s) to share common functionality like [run(),new()]
- [ ] Accept a generic type for the data attribute within the tasks yaml schema
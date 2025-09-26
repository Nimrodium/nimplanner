# nimplanner
cli and gui(?) application written in rust to keep track of plans and assignments
## features
* cli interface
* persistant state
* gtk? when no subcommand or --gui passed
* interactive

## CLI interface
generally, each subcommand operates on a task, and flags can be appended to the operation to influence their output
### subcommands
* `add <task>`
> adds a new task
* `rm <task>`
> deletes a task
* `edit <task>`
> enters interactive editing interface
* `mark <task>`
> changes status of task
* `ls <task>`
> list tasks


### flags
grouping flags
* `--group <group>` | `-g`
* `--important` | `-i`

status flags
* `--complete` | `-c`
* `--inprogress` | `-p`
* `--notstarted` | `-s`
* `--incomplete` | `-o`

due flags
* `--today`
* `--tomorrow`
* `--late`
* `--upcoming`
* `--due <int>`

misc flags
* `--gui`
* `--help` | `-h`

### usage
* creating a new task due tomorrow

```sh
    nplan add CSA-2.1 --tomorrow
```
* mark a task as complete
```sh
    nplan mark CSA-2.1 --complete
```

* delete a task
```sh
    nplan rm CSA-2.1
```

* list all tasks
```sh
    nplan ls
```
* list all important tasks
```sh
    nplan ls --important
```
* list all late tasks
```sh
    nplan ls --late
```
* list all incomplete upcoming tasks
```sh
    nplan ls --incomplete
```

# GUI 
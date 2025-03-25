# Simple CLI task manager

## Adding new task
```
task-cli add "Task Description"
```

## Updating task 
```
task-cli update 1 "New Task Description"
```

## Deleting task
```
task-cli delete 1
```

## Listing tasks

### List all tasks
```
task-cli list
```

### List TODO tasks
```
task-cli list-todo
```


### List INPROGRESS tasks
```
task-cli list-in-progress
```

### List DONE tasks
```
task-cli list-done
```

## Marking tasks

### Mark INPROGRESS
```
task-cli mark-in-progress 1
```

### Mark DONE
```
task-cli mark-done 1
```

# MemoMate
A simple, lightweight CLI for periodical notifications

## Usage
- `create` - create a new notification
```shell
# This command will create a notification with the title "Drink some water" that will be shown every 300 seconds
memo-mate create -t "Drink some water" -i 300
```

- `list` - list all notifications
- `delete` - delete the notification by ID (Ids are shown in the output of the `list` command)
- `start` - start sending notifications

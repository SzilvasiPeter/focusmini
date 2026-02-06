Focusmini is a compact Pomodoro CLI that alternates between work/break intervals, beeps on transitions.

- ~100 lines of code, ~569 KB stripped binary, no unsafe call.

Run `focusmini --help` for the options:

```
Usage: focusmini [OPTIONS]

Options:
  -w, --work <work_minutes>     Work interval length in minutes [default: 60]
  -b, --break <break_minutes>   Break interval length in minutes [default: 10]
  -a, --amplify <amplify>       Beep loudness multiplier [default: 1]
  -h, --help                    Print help information
  -V, --version                 Print version information
```

Adjust the flags if you need shorter intervals or louder cues.

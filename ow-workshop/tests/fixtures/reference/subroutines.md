# subroutines

Subroutine declaration, index allocation, calls, Start Rule, and the restart-with-wait warning.

Expected output: `subroutines.txt` (OverPy v9.7.20, `spoo-bar/overpy@e349a32`, language en-US).

## Source (reference only)

The original OverPy program. It is not a supported input; read it to write the equivalent `Workshop` impl.

```text
#!optimizeForSize

subroutine foo
subroutine bar 1
subroutine sub_with_wait_nested

def foo():
    @Name "my subroutine"
    
    A++
    bar()
    async(Sub0, AsyncBehavior.NOOP)
    async(sub_with_wait_nested, AsyncBehavior.NOOP)
    async(sub_with_wait_nested, AsyncBehavior.RESTART) #should throw warning

def bar():
    pass

def Sub0():
    B++


def sub_with_wait():
    A = 2
    wait()

def sub_with_wait_nested():
    A = 3
    sub_with_wait()
```

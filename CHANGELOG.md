# v0.2.0

## Changes:
Note that all dates are in DD/MM/YYYY form.
The below list grows downwards (thus, recent changes are put at the bottom)

### 10/06/2021
* initial commit
* added all supporting files (license, readme, contributing, changelog)
* basic framework implemented and examples of usage added (viewable in the examples folder)

### 10/06/2021
* able to embed a "state-store" inside of a component
* this state-store is a dyn trait object, and is dynamically downcast-able to whatever underlying concrete state-store
* messaging system has been consolidated inside of the "port" struct
    * each component has a port
    * each port contains connections to other component's ports
    * ports can drive messages to other components, where they will be handled by the handler function

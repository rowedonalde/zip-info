python
print("---- Loading Rust pretty-printers ----")

import os
sys.path.insert(0, os.getcwd() + "/etc/")

import gdb_rust_pretty_printing
gdb_rust_pretty_printing.register_printers(gdb)

end


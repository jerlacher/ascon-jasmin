require Ascon128.
require Ascon128a.

import var Ascon128.M.
import var Ascon128a.M.


(* Decryption results in a return value of 0 or 1 *)
hoare relate _m _c _clen _a _alen : Ascon128.M.ascon_128_a_decrypt : arg=(_m, _c, _clen, _a, _alen) ==> res=0 \/ res=1.
proof. proc. inline *. sim. qed.


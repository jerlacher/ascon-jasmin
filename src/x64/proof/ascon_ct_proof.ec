require Ascon128_ct.
require Ascon128a_ct.

import var Ascon128_ct.M.
import var Ascon128a_ct.M.

(* Ascon128 *)
(* Constant time encryption *)
equiv ascon128_encrypt_ct :
  Ascon128_ct.M.ascon_128_encrypt ~ Ascon128_ct.M.ascon_128_encrypt :
  ={leakages, m, c, mlen, a, alen, p} ==> ={leakages}.
proof. proc; inline *; sim. qed.

(* Constant time decryption *)
equiv ascon128_decrypt_ct :
  Ascon128_ct.M.ascon_128_decrypt ~ Ascon128_ct.M.ascon_128_decrypt :
  ={leakages, m, c, clen, a, alen, p} ==> ={leakages}.
proof. proc; inline *; sim. qed.

(* Ascon128a *)
(* Constant time encryption *)
equiv ascon128a_encrypt_ct :
  Ascon128a_ct.M.ascon_128_a_encrypt ~ Ascon128a_ct.M.ascon_128_a_encrypt :
  ={leakages, m, c, mlen, a, alen, p} ==> ={leakages}.
proof. proc; inline *; sim. qed.

(* Constant time decryption *)
equiv ascon128a_decrypt_ct :
  Ascon128a_ct.M.ascon_128_a_decrypt ~ Ascon128a_ct.M.ascon_128_a_decrypt :
  ={leakages, m, c, clen, a, alen, p} ==> ={leakages}.
proof. proc; inline *; sim. qed.

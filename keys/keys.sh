openssl genpkey -algorithm ed25519 -outform PEM -out private.pem
openssl pkey -in private.pem -outform PEM -pubout -out public.pem
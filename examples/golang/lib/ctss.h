#include <stdint.h>

const char *keygen(const char *server_url,
                   const char *room,
                   uint16_t index,
                   uint16_t threshold,
                   uint16_t number_of_parties);

const char *sign(const char *server_url,
                 const char *room,
                 const char *local_share,
                 const char *parties,
                 const char *data);

AC_INIT([Telluric Deckay],
  m4_esyscmd([grep version Cargo.toml | awk '{print $3}' | tr -d '"' | tr -d "\n"]),
  [https://github.com/TelluricDeckay/telluricdeckay/issues],
  m4_esyscmd([grep name Cargo.toml | awk '{print $3}' | tr -d '"' | tr -d "\n"]),
  [https://github.com/TelluricDeckay/telluricdeckay])

VERSION=$(grep version Cargo.toml | awk '{print $3}' | tr -d '"' | tr -d "\n")
AC_SUBST(VERSION)

DATE=$(date)
AC_SUBST(DATE)

AC_CONFIG_FILES([Makefile
                 telluricdeckay.desktop
                 telluricdeckay.appdata.xml
                 src/config_h.rs])
AC_OUTPUT

#!/bin/sh

# hash string like Java String::hashCode
hash_string() {
  str="${1:-}" h=0
  while [ -n "$str" ]; do
    h=$(( ( h * 31 + $(LC_CTYPE=C printf %d "'$str") ) % 4294967296 ))
    str="${str#?}"
  done
  printf %x\\n $h
}

hash_string "https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.8.6/apache-maven-3.8.6-bin.zip"
hash_string "apache-maven-3.8.6"

distributionSha256Sum=""
while IFS="=" read -r key value; do
  case "$key" in
    distributionUrl) distributionUrl="${value:-$distributionUrl}";;
    distributionSha256Sum) distributionSha256Sum="${value:-distributionSha256Sum}";;
  esac
done < ~/github.com/libtorch-bundle/.mvn/wrapper/maven-wrapper.properties

echo "distributionUrl: $distributionUrl"
echo "distributionSha256Sum: $distributionSha256Sum"
hash_string "$distributionUrl"

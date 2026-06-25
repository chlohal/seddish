

TESTNUM=0
testing()
{
  NAME="$1"
  [ -n "$1" ] || NAME="$2"

  TESTNUM=$(node -p "1+$TESTNUM")

  if [ $# -ne 5 ]
  then
    echo "Test $NAME has wrong number of arguments: $# (must be 5)" >&2
    exit 1
  fi

  echo -ne "///Automatically transformed from Busybox sed test $NAME\n"
  echo -ne "#[test]\nfn busybox_test_$TESTNUM() {\n"
  
  echo 'let expected = "' $3 '";';
  echo 'let input = "' $5 '";';

  echo $2 | sed "$ { s/sed '/let source = "'"'"/; s/'$/"'"'";/; }; $ ! N"

  echo 'test_from_busybox_sed(expected, input, source);'

  echo -e "}\n"
}
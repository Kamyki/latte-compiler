This is an university project for compiler class. I have uploaded the original
code only modifying this README.

The class was very demanding and time consuming, and I've listed possible
improvements to the project at the end of the file.

[Project description](https://www.mimuw.edu.pl/~ben/Zajecia/Mrj2018/latte-en.html)
and [language description](https://www.mimuw.edu.pl/~ben/Zajecia/Mrj2018/Latte/description.html).


Latte

Students, Srodowisko
====================
```
$ rustc --version
```
Lokalnie pracuje z dosc nowa wersja (1.31.1). Zalecana jest aktualizacja
kompilatora (nie trwa to dlugo), poniewaz moj kod miejscami korzysta
z niedawnych zmian w kompilatorze Rusta.

Instalacja Rusta (na wszelki wypadek):
```
$ curl https://sh.rustup.rs -sSf | sh
```
Aktualizacja Rusta:
```
$ rustup update
```
Zbudowanie projektu:
```
$ make
```
`latc` jest skrotem do `latc_llvm --make-executable`.

Uruchamialem u siebie lokalnie testy oficjalne i te dodatkowe od studentow -
wszystko dziala tak, jak powinno.

Runtime zostal napisany w C/C++ i znajduje sie w lib/runtime.cpp. Jest on
juz skompilowany do plikow *.ll i *.bc przy pomocy clanga. Sposob jego
kompilacji znajduje sie w `compile-runtime.sh`.

Na studentsie niestety nie ma zainstalowanych biblioteki standardowej C
32-bitowej, na ktorych pracowalem lokalnie, wiec produkuje binarki 64-bitowe.


Podjete decyzje
===============

- zmienilem skladnie dla indeksowania tablicy: `arr.[idx]`
- nie wspieram jawnego rzutowania
- wystepuja niejawne konwersje: podklasy do nadklasy (w tym tablicy dowolnie
  wymiarowej podklasy do tablicy o tym samym wymiarze nadklasy), oraz nulla
  do klasy lub tablicy,
- `for(int x : arr) {...}` jest rownoznaczny (semantycznie):
```
int i = 0;
while (i < arr.length) {
  // zmienna lokalna jedyna w swoim zakresie widocznosci,
  // i w zaleznosci od typu - operujemy na kopii lub referencji
  int x = arr.[i];
  {...}
}
```
w tym mozna napisac: `for(SubClass[] it : superclass_array_2d)`
- optymalizacja: petla foreach pod spodem jest jednakze zoptymalizowana
  i odpowiada takiemu kodu w C:
```
int *it = array, *end = array + length;
while (it < end) {
  int elem = *it;
  it++;
  ... // cialo petli
}
```
- dopuszczam martwy kod (np. `if (true)` czy `while (false)`)
- optymalizacja: nie generuje kodu dla martwej galezi if-a ani ciala while'a,
  jesli warunek petli jest falszywy, ani kodu po `while (true)` (obliczam
  wyrazenia stale, ktore nie zawieraja zmiennych),
- brak sztucznych ograniczen przy wyrazeniach - np. mozna odwolac sie do pola
  obiektu, gdy obiekt jest wynikiem wyrazenia, a nie tylko zmienna, podobnie
  przy tablicach
- programista odpowiada za weryfikacje, czy referencja do obiektu lub tablicy
  nie jest nullem, wpp. zachowanie jest niezdefiniowane (prawdopodobnie bedzie
  segfault),
- jesli elementami tablicy sa obiekty klasy lub inne tablice (tablice
  wielowymiarowe), to sa one przechowywane przez referencje i domyslnie
  zerowane:
```
MyClass[] arr = new MyClass[42];
// typeof(arr[4]) == MyClass
// arr[4] == null // domyslna wartosc

int[][] v = new int[][42];
// typeof(v[2]) == int[]
// v[2] == null // domyslna wartosc
```
- string jest typem referencyjnym, ale semantycznie nigdy nie jest nullem
  (domyslnie jest napisem pustym); jednakze napis pusty jest reprezentowany
  w implementacji przez wskaznik na adres 0,
- optymalizacja: dany napis jest generowany jako stala w LLVM-ie tylko raz
  i wspoluzywana przez wiele funkcji; w szczegolnosci nie jest generowana
  stala dla napisu pustego, poniewaz zawsze jest on reprezentowany przez
  wskaznik na adres 0,
- poprawnym jest:
```
void foo() {}
void bar() {return foo();}
```
- funkcje z runtime'u moga wywolac funkcje error() w przypadku wystapienia
  bledu, m.in. niepoprawnego formatu liczby czy ujemnej ilosci pamieci
  do zaalokowania,
- wszelkie symbole (funkcje, zmienne, klasy) wspoldziela przestrzenie nazw,
  tzn. nie mozna miec klasy i globalnej funkcji o tej samej nazwie ani
  pola w klasie i metody o tej samej nazwie,
- w przypadku dlugiego literalu liczbowego, parser moze sie scrashowac,
- w metodach jest dostepna zmienna `self` bedaca wskaznikiem na aktualny
  obiekt,
- dzielenie przez zero w wyrazeniach stalych jest wykrywane w czasie
  parsowania (mozna dostac syntax error i obok dzielenie przez 0),
- optymalizacja: inkrementacja i dekrementacja (operatory ++ i --)
  np. `x.foo().a.[4]++` tylko raz obliczy adres elementu `x.foo().a.[4]`,
- optymalizacja: dla tablic wykonuje tylko jedna alokacje; pole length
  znajduje sie w pamieci tuz przed elementami tablicy,
- generowany kod jest w postaci SSA: wszystkie zmienne lokalne
  sa w rejestrach, a load i store uzywane sa wylacznie do danych na stercie,
- optymalizacja: funkcje phi w bloku po ifie emituje tylko dla tych zmiennych
  ktore maja rozne wartosci (stala lub rejestr) w zaleznosci od bloku
  poprzednika; w przypadku while'a emituje funkcje phi dla wszystkich
  zmiennych lokalnych (chce uniknac (1) dodatkowej globalnej analizy,
  jakie zmienne wystepuja, bo to czasochlonne dla programisty,
  i (2) przechodzenie przez cialo bloku dwukrotnie, bo gdy takie bloki
  zagniezdzimy to mamy algorytm wykladniczy; robienie pelnej kopii zmiennych
  lokalnych takze jest wykladnicze, ale cos trzeba bylo wybrac),
- optymalizacja: jesli da sie tego uniknac, nie generuje kodu dla ! (negacji
  logicznej), tzn. w przypadku, gdy wynik decyduje gdzie skoczyc, a nie jest
  zapisywany na zmienna,
- w kodzie LLVM-a tworze nowe bloki dla syntaktycznych blokow w kodzie Latte
  (uwaga: petle i ify maja u mnie blok w AST, a nie instrukcje - nawet jesli
  w tekscie programu nie ma znakow {}), stad w grafie przeplywu sterowania
  moze wystepowac w wygenerowanym kodzie dluga sciezka bez rozgalezien,
- kompiluje na architekture 64-bitowa (maszyna students nie ma 32-bitowego
  runtime'u libc uzywanego przez clanga),
- przy alokacji tablicy, z gory znam rozmiary typow podstawowych (w tym
  wskaznik rowniez do nich zaliczam) - zakladam domyslny data layout dla
  64-bitowej architektury; dla alokacji obiektow, korzystam
  z "getelementptr null, 1",
- zaimplementowalem metody wirtualne,
- po refaktoryzacji: frontend dodaje odpowiednie niejawne rzutowania typow,
- po refaktoryzacji: frontend dodaje niejawne "this." tam, gdzie w srodku
  metod odwolujemy sie do skladowych klasy,


Drobne uwagi
============

- generator parserow lalrpop, z ktorego korzystam, nie wspiera komentarzy,
  wiec recznie je usuwam przed przekazaniem kodu do parsera (testowalem,
  ale zawsze moglem cos przeoczyc),
- staram sie wypisac tyle bledow na raz ile sie da,
- 1 pkt za SSA to zdecydowanie za malo; llvm duzo wymaga od IR-u, przez co
  zaimplementowanie wszystkich zmiennych lokalnych na funkcjach phi wymagalo
  duzo nowego kodu, aby kompilator llvm-a byl zadowolony


Opis kompilacji runtime'u znajduje sie w `compile-runtime.sh`
Opis kompilacji reszty znajduje sie w `src/main.rs`


Mozliwe usprawnienia
====================

Mozliwe usprawnienia po refaktoringu frontendu (moze on modyfikowac AST):

- przenazwanie zmiennych - wtedy nie bedzie potrzebne dodatkowa logika
  w backendzie z proxy env,
- usuniecie martwego kodu (po returnie, if/while true/false) [1],

Inne mozliwe usprawienia:

- duzo malych kawalkow kodu sie powtarza - mozna przygotowac makra,
- mozna w specjalny sposob obslugiwac proxy env w backendzie dla SSA:
  patrzec na numer warstwy lub dodatkowy atrybut przy zapisie i w ten sposob
  wyeliminowac zbedna pelna kopie wszystkich zmiennych lokalnych (bedzie miec
  czas amortyzowany liniowy zamiast wykladniczego - bedzie obslugiwac tylko
  te zmienne, ktore faktycznie wystapily w srodku bloku),
- skladnia dla tablic `a[idx]` zamiast `a.[idx]`,
- [1] mozna sprobowac uzyc `Rc<RefCell<_>>` zamiast `Box<_>` - moze to dodac
  wiecej wskaznikow posrednich, ale w momencie modyfikowania drzewa AST
  nie bedzie potrzebne kopiowanie calego podwyrazenia, zeby tylko zadowolic
  borrow checkera (komponent w kompilatorze Rusta),
  _Note: podczas robienia refaktoringu, robie kopie obiektow w wielu miejscach,
        gdzie wczesniej byly referencje - i jest szybciej. Byc moze poprzez
        wyeliminowanie pointer chasingu._

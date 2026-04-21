# Objasnjenje

```
#![cfg_attr(not(feature = "std"), no_std)]
```

Ovo u Substrate znači: "Ako ne kompajliraš za standardni Rust (std), onda koristi no_std okruženje." <br>
`no_std` kaže Rust-u: "ne koristi standardnu biblioteku (std), radi u “bare metal / blockchain” okruženju"

```
cfg_attr(...)
```

znači: “ako je uslov ispunjen → primeni ovo”

```
not(feature = "std")
```

znači: ako NE koristiš std feature (npr. runtime na chain-u)

> Zašto je to potrebno u Substrate-u? <br>
> U Substrate pallet-ima: <br>
>
> - kod se izvršava na blockchain-u
> - nema OS-a
> - nema file systema
> - nema standardnih Rust stvari
>
> zato mora no_std <br>
> Kad se kod pokreće na blockchain-u, koristi minimalno okruženje bez standardne Rust biblioteke

> Ukratko <Br>
> Ova linija omogućava da tvoj Substrate kod radi u blockchain okruženju gde nema standardne Rust biblioteke (std).

---

```
pub use pallet::*;
```

U Substrate ovo znači: “Re-exportuj (ponovo izloži) sve iz modula pallet da budu dostupni spolja kao da su u ovom fajlu.”

`pallet::*` = svi tipovi, funkcije, eventi, storage itd. unutar modula pallet <br>
`pub use` = čini ih javnim van ovog fajla

Obo se koristi da ne moras da pises:

```
pallet::Pallet
pallet::Config
pallet::Event
```

nego mozes direktno:

```
Pallet
Config
Event
```

Kao da kažeš: “Uzmi sve iz unutrašnje kutije pallet i stavi ih na sto da svako može da ih koristi direktno.”

`pub use pallet::*;` omogućava da svi delovi pallet-a budu dostupni spolja bez dodatnog prefix-a pallet::.

---

```
#[frame_support::pallet]
pub mod pallet {
```

`#[frame_support::pallet]` je makro koji kaže Substrate FRAME sistemu da je ovaj modul pallet (blok logike za blockchain runtime) i da treba automatski da generiše sav potrebni boilerplate kod (storage, calls, events, errors). <br>
`pub mod pallet {` samo definiše Rust modul u kojem se nalazi cela logika tog pallet-a. <br>

> Ukratko: <br>
> ova linija označava početak Substrate pallet-a i uključuje automatsku generaciju blockchain funkcionalnosti iz tog modula.

<br>

<br>

```
use frame_support::{pallet_prelude::*, dispatch::DispatchResult, ensure};
use frame_system::pallet_prelude::*;
```

Ovo su importi koji ti dovode potrebne alate za Substrate pallet.

`use frame_support::{pallet_prelude::*, dispatch::DispatchResult, ensure};` <br>
`pallet_prelude::*` → uvozi osnovne tipove i makroe za pisanje pallet-a (Storage, Event, Error, itd.) <br>
`DispatchResult` → tip koji kaže da funkcija može da uspe ili vrati grešku <br>
`ensure!` → makro za proveru uslova (ako nije tačno, prekida izvršavanje i vraća error)

`use frame_system::pallet_prelude::*;` <br>
uvozi osnovne stvari iz system modula Substrate-a <br>
Tu su tipovi za:

- `Origin` (ko poziva funkciju)
- `AccountId`
- `ensure_signed` (da proveriš ko je potpisao transakciju)

Prvi import daje “alat za pisanje pallet logike”, a drugi daje “osnovne blockchain/system stvari kao što su nalog i potpisivanje”.

<br>

<br>

```
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }
```

`#[pallet::config]` - Kaže Substrate FRAME-u da je ovo Config trait za ovaj pallet. <br>
Config trait u Substrate pallet-u definiše koje zavisnosti i tipove runtime mora da obezbedi tom pallet-u i omogućava mu da se poveže sa sistemom (npr. accounti i eventi). <br>

`pub trait Config: frame_system::Config` <br>
Definiše trait koji mora da implementira runtime.<br>
`frame_system::Config` → znači da pallet zavisi od osnovnog Substrate sistema (accounti, origin, itd.)

`type RuntimeEvent` <br>
Ovo je tip koji povezuje evente ovog pallet-a sa globalnim runtime event sistemom. <br>
`From<Event<Self>>` → omogućava da se događaji iz ovog pallet-a konvertuju u runtime event <br>
`IsType<...>` → osigurava da je tip kompatibilan sa runtime event sistemom

`+` - Znak + u ovom kontekstu znači da tip mora da ispunjava više trait-ova (ograničenja) istovremeno. Taj tip mora da može i da se konvertuje iz Event i da bude kompatibilan sa određenim runtime tipom u isto vreme.

Ovo kaže: “Runtime mora da zna kako da handle-uje evente ovog pallet-a i mora da obezbedi osnovni Substrate system config.”

> Ukratko:
> Config trait definiše zavisnosti pallet-a i povezuje njegove evente sa globalnim runtime event sistemom.

<br>

<br>

```
 #[pallet::pallet]
    pub struct Pallet<T>(_);
```

Ovo u Substrate definiše glavnu strukturu pallet-a koja predstavlja runtime instancu tog pallet-a.

`#[pallet::pallet]` → FRAME makro koji od ove strukture pravi “pallet runtime objekat” <br>
`pub struct Pallet<T>` → generička struktura koja radi sa runtime konfiguracijom (T) <br>
`(_)` → placeholder jer Substrate sam generiše internu strukturu (ne koristiš polja ručno)

Ovo je “glavni entry objekat” tvog pallet-a koji Substrate koristi da poveže storage, calls i evente u jednu celinu.

> Ukratko: <br>
> Ova struktura predstavlja sam pallet u runtime-u i FRAME je koristi kao osnovni objekat za izvršavanje logike.

> Runtime <br>
> Runtime u Substrate je deo blockchaina koji sadrži svu logiku sistema (pallet-e) i definiše pravila kako se transakcije izvršavaju. <br>
> Ukratko: to je “mozak” blockchaina koji odlučuje šta je validno i kako se stanje menja.

<br>

<br>

```
// STORAGE
    #[pallet::storage]
    pub type Balances<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u128, ValueQuery>;

    #[pallet::storage]
    pub type Accounts<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;
```

Ovo definiše podatke koje tvoj pallet čuva na blockchainu u Substrate.

`#[pallet::storage]` - Kaže FRAME-u da je ovo on-chain storage (stanje koje se čuva u blockchainu).

1. `Balances`

```
pub type Balances<T: Config> =
    StorageMap<_, Blake2_128Concat, T::AccountId, u128, ValueQuery>;
```

Ovo znači:

- mapa (key → value struktura)
- ključ = `AccountId`
- vrednost = `u128` (balance)
- čuva koliko para svaki account ima
- `ValueQuery` znači da ako nema vrednosti → vraća 0

> `Balances<T: Config> <br>` <br>
> T = generički runtime tip. T predstavlja ceo runtime environment. T je “ceo sistem” (runtime context) <br>
> iz njega dolazi: `T::AccountId`, `T::BlockNumber` i `T::RuntimeEvent` <br>
> `Config` - trait koji runtime mora da implementira

> Parametri `StorageMap`- a <br>
> `StorageMap<Hasher, Key, Value, QueryKind>` <br>
> U tvom slučaju: `StorageMap<_, Blake2_128Concat, T::AccountId, u128, ValueQuery>` <br>
>
> Redom šta znači: <br>
>
> 1. `_` - FRAME sam izabere storage prefix
> 2. `Blake2_128Concat` - HASH funkcija za key. To je hashing mehanizam za storage keys
> 3. `T::AccountId` - KEY (ključ u mapi), npr. wallet address
> 4. `u128` - VALUE (šta se čuva), npr. balance
> 5. `ValueQuery` - ponašanje kada nema vrednosti, vrati default (0 za u128), umesto `Option`

2. `Accounts`

```
pub type Accounts<T: Config> =
    StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;
```

Ovo znači:

- mapa (account → da li postoji ili ne)
- ključ = `AccountId`
- vrednost = `bool`
  - `true` = account postoji
  - `false` = ne postoji
- koristi se da proveriš da li je korisnik registrovan

> Ukratko: <br>
> `Balances` → čuva koliko ko ima novca <br>
> `Accounts` → čuva da li korisnik postoji u sistemu <br>
> oba su on-chain storage mape (trajni blockchain podaci)

<br>

<br>

```
#[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AccountCreated(T::AccountId),
        DepositMade(T::AccountId, u128),
        TransferCompleted(T::AccountId, T::AccountId, u128),
    }
```

`#[pallet::event]` <br>
Kaže FRAME-u da je ovo event sistem ovog pallet-a — sve što se ovde definiše može biti emitovano na chain-u.

`#[pallet::generate_deposit(...)]` <br>
Ovo automatski generiše funkciju `deposit_event(...)` koju koristiš da “pošalješ event” u runtime log. <br>

`pub enum Event<T: Config>` <br>
Ovo je lista svih događaja koje tvoj pallet može da emituje: <br>

1. `AccountCreated(T::AccountId)` - okida se kada neko napravi nalog
2. `DepositMade(T::AccountId, u128)` - okida se kada neko uplati novac
3. `TransferCompleted(T::AccountId, T::AccountId, u128)` - okida se kada se izvrši transfer: `from`, `to` i `amount`

Šta su events u praksi? <br>
To su logovi na blockchainu <br>
Frontend ih čita

> Ukratko: <br>
> Event u Substrate-u je zapis o tome šta se desilo u pallet-u, koji se emituje na blockchainu radi praćenja i frontend upotrebe.

<br>

<br>

```
 #[pallet::error]
    pub enum Error<T> {
        AccountAlreadyExists,
        AccountNotFound,
        NotEnoughBalance,
    }
```

`#[pallet::error]` <br>
Kaže FRAME-u: “Ovo su sve moguće greške ovog pallet-a”

`pub enum Error<T>` <br>
Ovo je lista situacija kada nešto NE može da se izvrši:

1. `AccountAlreadyExists` - pokušavaš da napraviš account koji već postoji
2. `AccountNotFound` - account ne postoji u storage-u
3. `NotEnoughBalance` - nema dovoljno sredstava za operaciju

Kako se koristi u kodu:

```
ensure!(
    balance >= amount,
    Error::<T>::NotEnoughBalance
);
```

ako uslov nije ispunjen → prekida se izvršavanje i vraća error

> Ukratko: <br>
> Error enum u Substrate pallet-u definiše sve greške koje mogu da se dese i koristi se za prekid izvršavanja uz jasan razlog.

<br>

<br>

`#[pallet::call]` - Ovo u Substrate označava deo pallet-a gde definišeš funkcije koje korisnici mogu da pozovu (`extrinsics`). <br>
Konkretno znači: “Sve funkcije ispod ovog atributa su javni pozivi na blockchainu” <br>
U ovom delu pišeš:

- create_account
- deposit
- transfer

tj. sve što korisnik može da izvrši kao transakciju

Svaka funkcija u `#[pallet::call]` postaje extrinsic (blockchain transaction)

> Ukratko: <br>
> `#[pallet::call]` označava deo pallet-a koji sadrži funkcije koje korisnici mogu da pozovu kao transakcije na blockchainu.

> `Extrinsic` u Substrate je spoljašnji poziv (transakcija) koji dolazi u blockchain i menja njegovo stanje.

<br>

`impl<T: Config> Pallet<T> {` - Implementacija funkcija za Pallet koji radi sa generičkim runtime-om `T` koji mora da ispunjava Config trait.” <br>

- `impl` → implementacija (definišeš funkcije)
- `T` → ceo runtime (blockchain okruženje)
- `: Config` → T mora da ima sve što si definisao u Config trait-u
- `Pallet<T>` → tvoj konkretan pallet u tom runtime-u

Unutar toga pises: helper funkcije, internal logiku i event helper-e (ali NE extrinsics (oni su u #[pallet::call])) <br>

> Ukratko: <br>
> `impl<T: Config> Pallet<T>` definiše implementaciju funkcija za pallet koje rade u okviru runtime-a koji mora da ispunjava Config pravila (#[pallet::config]).

> Zasto je svuda referenca kod who (&who)? <br>
> Zato što želiš da radiš nad istim korisnikom bez kopiranja njegove vrednosti. <br>
>
> ```
> Accounts::<T>::get(&who);
> Balances::<T>::insert(&who, balance);
> ```
>
> `&who` znači da prosleđuješ referencu na `AccountId`, ne kopiraš podatak svaki put <br>
>
> Zašto se koristi referenca? <br>
>
> 1. efikasnost - nema kopiranja (bitno za veće tipove)
> 2. API zahteva referencu - mnoge Substrate funkcije očekuju &T::AccountId
> 3. Rust pravilo - funkcije često primaju reference da bi bile brže i sigurnije
>
> `who` = vlasnik <br>
> `&who` = pokazuješ gde je taj vlasnik u memoriji

 <br>

```
 #[pallet::weight(10_000)]
        pub fn create_account(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                !Accounts::<T>::get(&who),
                Error::<T>::AccountAlreadyExists
            );

            Accounts::<T>::insert(&who, true);
            Balances::<T>::insert(&who, 0);

            Self::deposit_event(Event::AccountCreated(who));
            Ok(())
        }
```

`#[pallet::weight(10_000)]` - definiše “težinu” transakcije (koliko resursa troši). <br>
Koristi se za: naplatu fee-a i procenu koliko je operacija skupa

`pub fn create_account(...) -> DispatchResult` - funkcija koju korisnik može da pozove <br>
`DispatchResult` znači: uspeh ili greška

`let who = ensure_signed(origin)?;` - proverava da li je poziv potpisan i uzima ko je pozvao funkciju

`ensure!(!Accounts::<T>::get(&who), ...)` - proverava da li account već postoji, onda baca grešku

`Accounts::<T>::insert(&who, true);` - upisuje u storage da ovaj account sada postoji

`Balances::<T>::insert(&who, 0);` - inicijalizuje balans na 0

`Self::deposit_event(Event::AccountCreated(who));` - emituje event:`AccountCreated`

`Ok(())` - znaci da je sve prošlo uspešno

> Ukratko: <br>
> Ova funkcija proverava ko je pozvao, sprečava duplikat account-a, kreira novi account sa balansom 0 i emituje event.

<br>

```
 #[pallet::weight(10_000)]
        pub fn deposit(origin: OriginFor<T>, amount: u128) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(Accounts::<T>::get(&who), Error::<T>::AccountNotFound);

            let balance = Balances::<T>::get(&who);
            Balances::<T>::insert(&who, balance + amount);

            Self::deposit_event(Event::DepositMade(who, amount));
            Ok(())
        }
```

`#[pallet::weight(10_000)]` - određuje koliko je operacija “teška” (za fee i resurse).

`let who = ensure_signed(origin)?;` - uzima ko je pozvao funkciju (mora biti potpisan korisnik) <br>
“Potpisan korisnik” znači da je transakcija kriptografski potpisana privatnim ključem tog naloga. <br>
Šta to znači u praksi? To znaci da korisnik ima privatan kljuc, njime potpisuje transakciju i blockchain proverava potpis <br>

> Ukratko <br>
> Potpisan korisnik je onaj koji je dokazao identitet tako što je potpisao transakciju svojim privatnim ključem.

`ensure!(Accounts::<T>::get(&who), ...)` - proverava da li account postoji, ako ne postoji → error

`let balance = Balances::<T>::get(&who);` - uzima trenutni balans korisnika

`Balances::<T>::insert(&who, balance + amount);` - povećava balans za prosleđeni iznos

`deposit_event(...)` - emituje event da je uplata izvršena

`Ok(())` - sve prošlo uspešno

> Ukratko: <br>
> Funkcija proverava da li korisnik postoji, uzima njegov balans, dodaje iznos i upisuje novi balans uz emitovanje event-a.

<br>

```
 #[pallet::weight(10_000)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: u128,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;

            ensure!(Accounts::<T>::get(&from), Error::<T>::AccountNotFound);
            ensure!(Accounts::<T>::get(&to), Error::<T>::AccountNotFound);

            let from_balance = Balances::<T>::get(&from);
            ensure!(from_balance >= amount, Error::<T>::NotEnoughBalance);

            Balances::<T>::insert(&from, from_balance - amount);

            let to_balance = Balances::<T>::get(&to);
            Balances::<T>::insert(&to, to_balance + amount);

            Self::deposit_event(Event::TransferCompleted(from, to, amount));
            Ok(())
        }
```

`let from = ensure_signed(origin)?;` - uzima ko šalje novac (mora biti potpisan)

Provere: <br>

```
ensure!(Accounts::<T>::get(&from), Error::<T>::AccountNotFound);
ensure!(Accounts::<T>::get(&to), Error::<T>::AccountNotFound);
```

oba account-a moraju da postoje

<br>

Provera balansa:

```
let from_balance = Balances::<T>::get(&from);
ensure!(from_balance >= amount, Error::<T>::NotEnoughBalance);
```

proverava da li pošiljalac ima dovoljno novca

<br>

Skidanje sa sender-a:

```
Balances::<T>::insert(&from, from_balance - amount);
```

oduzima novac

<br>

Dodavanje primaocu

```
let to_balance = Balances::<T>::get(&to);
Balances::<T>::insert(&to, to_balance + amount);
```

dodaje novac

<br>

Event

```
Self::deposit_event(Event::TransferCompleted(from, to, amount));
```

> Ukratko: <br>
> Funkcija proverava account-e i balans, oduzima novac od pošiljaoca, dodaje primaocu i emituje event o transferu.

<br>

<br>

---

```
#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok};

    #[test]
    fn create_account_works() {
        // basic test skeleton (možemo kasnije proširiti)
        assert_eq!(1, 1);
    }
}
```

`#[cfg(test)]` - znaci da se ovaj kod kompajlira samo kada pokrenemo testove, ne ulazi u produkcioni kod

`mod tests { ... }` - poseban modul gde pišeš testove

`use super::*;` - uvozi sve iz tvog pallet-a da možeš da ga testiraš

`use frame_support::assert_ok;` - helper za testiranje - proverava da li funkcija vraća Ok(())

`#[test]` - označava jednu test funkciju

`fn create_account_works()` - naziv funkcije

`assert_eq!(1, 1);` - samo dummy test (“Dummy test” znači da je to lažni/test primer koji ne proverava ništa stvarno u tvom kodu. - uvek prolazi), služi kao placeholder - nikad neće pasti test (nema veze sa tvojim testom)

Ovo nije pravi test!

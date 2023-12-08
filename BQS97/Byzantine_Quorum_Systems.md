<!-- https://people.cs.umass.edu/~arun/cs691ee/reading/BQS97.pdf -->

## <center><code>Byzantine Quorum Systems</code></center>
#### <center><code>Dahlia Malkhi Michael Reiter</code></center>

###### <center><code>AT&T Labs—Research, Murray Hill, NJ USA<br>{dalia,reiter}@research.att.com</code></center>

### Abstract

Quorum Bystems are well-known tools for ensuring the con-<br>
sistency and availability of replicated data despite the be-<br>nign failure of data repositones. In this paper we consider the arbitrary (Byzantine) failure of data repositories and present the first study of quorum system requirements and construct ions that ensure data availabNit y and consistence y despite these failures. We also consider the load associated with our quorum systems, i.e., the minimal access probabil-<br>ity of the busiest server. For services subject to arbitrary failures, we demonstrate quorum systems over n servers with a load of `O(1/(sqrt(n))`, thus meeting the lower bound on load for
benignly fault-tolerant quorum systems. We explore several variations of our quorum systems and extend our construc- tions to cope with arbitrary client failures.

#### 1. Introduction

A well known way to enhance the availability and performance of a replicated service is by using quorums. A quorum system for a universe of servers is a collection of subsets of servers, each pair of which intersect. Intuitively, each quorum can operate on behalf of the system, thus increasing its availability and performance, while the intersection property guarantees that operations done on distinct quorums preserve consistency.

&ensp;&ensp;&ensp;&ensp;In this paper we consider the arbitrary (Byzantine) failure of clients and servers, and initiate the study of quorum systems in this model. Intuitively, a quomm system tolerant of Byzantine failures is a collection of subsets of servers, each pair of which intersect in a set containing sufficiently many correct servers to guarantee consistencey of the replicated data as seen by clients. We provide the following contributions.

`1.` We define the class of manking quorum systems, with which data can be consistently replicated in a way that is resiiient to the arbhmy fsilure of data repositories. We present several example constructions of such systems and show necessary and sufficient conditions for the existence of masking quorum systems under different failure assumptions.

`2.` We explore two variations of masking quorum systems. The first, called dimernination quorum syatema, is suited for services that receive and distribute sel~-ueri.fyirtg information from correct clients (e.g., digitally signed values) that faulty servers can fail to redistribute but cannot undetectable alter. The second variation, called opaque masking quorum systems, is similar to regular masking quorums in that it makes no assumption of self verifying data, but it differs in that clients do not need to know the failure scenarios for which the service was designed. This somewhat simplifies the client protocol and, in the case that the failures are maliciously induced, reveals less information to clients that could guide an attack attempting to compromise the system.

`3.` We explore the load of each type of quorum system, where the load of a quorum system is the minimal access prob- ability of the busiest server, minimizing over all strate- gies for picking quorums. We present a masking quorum system with the property that its load over a total of n Servem is 0(~), thereby meeting the lower bound for the load of benignly-fault-tolerant quorum systems. For opaque masking quorum systems, we prove a lower bound of ~ on the load, and present a construction that meets this lower bound and proves it tight.

&ensp;&ensp;&ensp;&ensp; For services that use masking quorums (opaque or not), we show how to deal with faulty clients in addition to faulty servers. The primary challenge raised by client fail- ures is that there is no guarantee that clients will update quorums according to any specified protocol. Thus, a faulty client could leave the service in an inconsistent and irrecoverable state. We develop an update protocol, by which clients access the replicated service, that prevents clients from leaving the service in an inconsistent state. The protocol has the desirable property that it involves only the quorum at which an access is attempted, while providing system-wide consistency properties.
569

`4.` In our treatment, we express assumptions about possi- ble failures in the system in the form of a fail-prone system B={ BI,..., Bh ) of servers, such that some Bi cent sins all the faulty servers. This formulation ineludes typical fail- ure assumptions that at most a threshold ~ of servers fail (e.g., the sets B1,..., Bk could be ~ sets of ~ servers), but it also generalizes to allow less uniform failure scenar- ios. Our motivation for exploring this generalization stems from our experience in constructing secure distributed ser- vices [34, 27], i.e., distributed services that can tolerate the malicious corruption of some (typically, up to a threshold number of) component servers by an attacker. A criticism to assuming a simple threshold of corrupted servers is that server penetrations may not be independent. For exam- ple, servers in physical proximity to each other or in the same administrative domain may exhibit correlated proba- bilities of being captured, or servers with identical hardware and software platforms may have correlated probabilities of electronic penetration. By exploiting such correlations (i.e., knowledge of the collection B), we can design quorum sys- tems that more effectively mask faulty servers.
Our quorum systems, if used in conjunction with ap- propriate protocols and synchronization mechanisms, can be used to implement a wide range of data semantics. In this paper, however, we choose to demonstrate a variable supporting read and write operations with relatively weak semantics, in order to maintain focus on our quorum con- structions. These semantics imply a sate variable [24] in the caae of a single reader and single writer, which a set of correct clients can use to build other abstractions, e.g., atomic, multi-writ er multi-reader registem [24, 21, 25], con- current timestamp systems [12, 19], l-exclusion [11, 2], and atomit snapshot scars [1, 5]. Our quorum constructions can also be directly exploited in algorithms that employ “uni- form” quorums for fault tolerance (by involving a threshold of processes), in order to improve efficiency or tolerate non- uniform failure scenarios. Examples include algorithms for shared memory emulation [6], randomized Byzantine agree- ment [39], reliable Byzantine multicast [8, 33, 27], and secure replicated data [18].

The rest of this paper is structured as follows. We begin in Section 2 with a description of related work. 

In Section 3 we present our system model and definitions. We present quorum systems for the replication of arbitrary data subject to arbltrarv server failures in 

Section 4. and in Section 5 “we present two variations of these systems. We then detail an access protocol for replicated services that tolerate faulty clients in addition to faulty servers in Section 6. We conclude in Section 7.

### 2 Related work

Our work was influenced by the substantial body of literature on quorum systems for benign failures and applications that make use of them, e.g., [15, 38, 26, 14, 17, 13, 9, 4, 30]. In particular, our grid construction of Section 4 was influ- enced by grid-like constructions for benign failures (e.g., [9]), and we borrow our definition of load from [30].
Quorum systems have been previously employed in the implementation of security mechanisms. Naor and Wool [31] described methods to construct an access-control service us- ing quorums. Their constructions use cryptographic tech- niques to ensure that out-of-date (but correct) servers can- not grant access to unauthorized users. Agrawal and El Abbadi [3] and Mukkamala [29] considered the confiden- tiality of replicated data despite the disclosure of the con- tents of a threshold of the (otherwise correct) repositories. Their constructions used quorums with increased intersec- tion, combined with Rabin’s dispersal scheme [32], to en- hance the confidentiality and availability of the data despite some servers crashing or their contents being observed. Our work differs from all of the above by considering arbitrarily faulty servers, and accommodating failure scenarios beyond a simpIe threshold of servers.

Herlihy and Tygar [18] applied quorums with increased intersection to the problem of protecting the confidential- ity and integrity of replicated data against a threshold of arbitrarily faulty servers. In their constructions, replicated data is stored encrypted under a key that is shared among the servers using a threshold secret-sharing scheme [36], and each client accesses a threshold number of servers to recon- struct the key prior to performing (encrypted) reads and writes. This construction exhibits one approach to make replicated data self-verifying via encryption, and thus the quorum system they develop is a special case of our dis- semination quorum systems, i.e., for a threshold of faulty servers.
570

### 3 Preliminaries

### 3.1 Preliminaries System model

We assume a universe .?Jof servers, IIYI= n, and an arbi- trary number of clients that are distinct from the servera. A quorum system ~ ~ 2U is a set of subsets of U, any pair of which ht ersect. Each Q c Q is called a quorum.

Servers (and clients) that obey their specifications are correct. A faulty server, however, may deviate from its spec- ification arbitrarily. A fail-prone system B c 2U is a set of subsets of U, none of which is contained in another, such that some B E B contains all the faulty servers. The fail- prone system represents an assumption characterizing the failure scenarios that can occur, and could express typical assumptions that up to a threshold of servers fail, as well as less uniform assumptions.

In the remainder of this section, and throughout Sec- tions 4 and 5, we assume that clients behave correctly. In Section 6 we will relax this assumption (and will be explicit when we do so).

We assume that any two correct processes (clients or servers) can communicate over an authenticated, reliable channel. That is, a correct process receives a message from another correct process if and only if the other correct pro- cess sent it. However, we do not assume known bounds on message transmission times; i.e., communication is asyn- chronous.

### 3.2 Access protocol

We consider a problem in which the clients perform read and write operations on a variable z that is replicated at each server in the universe U. A copy of the variable z is stored at each server, along with a timestamp value t.Timestamps are assigned by a client to each replica of the variable when the client writes the replica. Our protocols require that differ- ent clients choose different timestamps, and thus each client c chooses its timestamps from some set T. that does not intersect T,, for any other client c’. The timestamps in T. can be formed, e.g., as integers appended with the name of c in the low-order bits. The read and write operations are implement ed as follows.
Write: For a client c to write the value u, it queries each server in some quorum Q to obtain a set of value/timestamp pairs A = {<uti, tti>}ti~Q; chooses a timestamp t E Tc greater than the highest timestamp value in A end greater than any timestamp it has chosen in the past; and updates z and the associated timestamp at each server in Q to v and t, respectively.

Read: For a client to read x, it queries each server in some quorum Q to obtain a set of value/ timestamp pairs A = {<Vu,t.>}U~Q. The client then applies a deterministic function Restdto to A to obtain the result Result(A) of the read operation.
In the case of a write operation, each server updates its local variable and timestamp to the received values <u, t> only if t is greater than the timestamp currently associated with the variable.
Two points about this description deserve further dis-
cussion. First, the nature of the quorum sets Q and the function Result () are intentionally left unspecified; further clarification of these are the point of this paper. Second, this descrbtion is intended to reauire a client to obtain a set A containing value/timestamp pairs from every server in some quorum Q. That is, if a client is unable to gather a complete set A for a quorum, e.g., because some server in the quorum appears unresponsive, the client must try to perform the operation with a different quorum. This re- quirement stems from our lack of synchrony assumptions on the network: in general, the only way that a client can know that it has accessed every correct server in a quorum is to (apparently successfully) access every server in the quorum. Our framework guarantees the availability of a quorum at any moment, and thus by attempting the operation at mul- tiple quorums, a client can eventually make progress. In some cases, the client can achieve progress by incrementally accessing servers until it obtains responses from a quorum of them.

In Sections 4 and 5, we will argue the correctness of the above protocol—instantiated with quorums and a Resul to function that we will define according to the following semantics; a more formal treatment of these concepts can be found in [24]. We say that a read operation begins when the client initiates the operation and ends when the client obtains the read value; an operation to write value `u` with timestamp tbegins when the client initiates it and ends when all correct servers in some quorum have received the update `v, t`. An operation OPI precedes an operation o~ if opI ends before opa begins (in real time). If opI does not precede opa and opz does not precede opl, then they are called concurrent. Given a set of operations, a aertalization of those operations is a total ordering on them that extends the precedence ordering among them. Then, for the above protocol to be correct, we require that any read that is con- current with no writes returns the last value written in some serialization of the preceding writes. In the case of a single reader, single writer variable, this will immediately imply safe semantics [24].

### 3.3 Load

A measure of the inherent performance of a quorum system is its ioad. Naor and Wool [30] define the load of a quorum system as the probability of accessing the busiest server in the best case. More precisely, given a quorum system Q, an access strategu w is a probability distribution on the element 5 of Q; i.e.; ‘~QEQ w(Q) = 1: w(Q) is the probability
that quorum Q will be chosen when the service is accessed. Load is then defined as follows:
Definition 3.1 Let a strategy w be given for a quorum system Q={ Ql, ..., Q~} over a universe U. For an element u E U, the load induced by w on u is l~(u) = ~Q~3U w(Qi).

The load induced by a strategy w on a quorum system Q is `L(Q) = rnm{L(u)}`. The system load (or just load) on a quorum system Q is `L(Q) = m~{LW(Q)}`, where the minimum is taken over all strategies. ❑ We reiterate that the load is a best case definition. The load of the quorum system will be achieved only if an op- timal access strategy is used, and only in the case that no failures occur. A strength of this deilnition is that load is a property of a quorum system, and not of the protocol using it. A comparison of the definition of load to other seemingly plausible definitions is given in [30].

### 4 Masking quorum systems
571

In this section we introduce masking quorum systems,
can be used to mask the arbitrarily faulty behavior of data repositories. To motivate our definition, suppose that the replicated variable z is written with quorum QI, and that subsequently z is read using quorum Qz. 

If 13 is the set of arbitrarily faulty servers, then (QI n Qz ) \ 1? is the set of correct servers that possess the latest value for z. In order for the client to obtain this value, the client must be able to locate a value/timestamp pair returned by a set of servers that could not all be faulty. In addition, for availability we require that there be no set of faulty servers that can disable all quorums.

`4.1` A quorum system Q is a masking quorum aystern for a fail-prone system B if the following properties are satisfied.
Definition
M2, vBET33QEQ:
—
BnQ=O
It is not difficult to verify that a masking quorum sys- tem enables a client to obtain the correct answer from the service. The write operation is implemented as described in Section 3, and the read operation becomes:
Read: For a client to read a variable z, it queries each server
in some quorum Q to obtain a set of value/timestamp
A = {<vu, &>}@Q. The client computes the set A’ = {<v, t> : 3B+c _ Q[ VBEZ3[B+~B]A
pairs
Vu EB+[vu=v Atti=t] ]}.
The client then chooses the pair <v, t>in A’ with the highest timestamp, and chooses v as the result of the read operation; if A’ is empty, the client returns 1 (a null value).
Lemma 4.2 A read operation that is concurrent with no write operations returns the value written by the last pre- ceding write operation in some serialization of all preceding write operations.
which

 `Proof`. Let W denote the set of write operations preceding the read. The read operation will return the value written in the write operation in W with the highest timestamp, since, by the construction of masking quorum systems, this value/timestamp pair will appear in A’ and will have the highest timestamp in A’ (any pair with a higher timestamp will be returned only by servers in some B c B). So, it suf- fices to argue that there is a serialization of the writes in W in which this write operation appears last, or in other words, that this write operation precedes no other write operation in W. This is immediate, however, as if it did precede an- other write operation in W, that write operation would have a higher timest amp. ❑
This lemma implies that the protocol above implements a single-writer single-reader safe variable [24]. From these, multi- writ er multi-reader at omit variables can be built using well-known constructions [24, 21, 25].
A necessary and sufficient condition for the existence of a masking quorum system (and a construction for one, if it exists) for any given fail-prone system B is given in the following theorem:
Theorem 4.3 Let B be a fail-prone system for a universe U. Then there exists a masking quomm system for B iff 4? = {U\ B : B ~ f3} is a masking quorum system for B.
Proof. Obviously, if Q is a masking quorum system for B, then one exists. To show the converse, assume that Q is not a masking quorum. Since M2 holds in Q by construction, there exist QI, QZ E Q and B’, B“ ●B, such that (Ql n Qz)\B’ c B”. Let Bl=U\Qland BZ=U\QZ. By the construction of Q, we know that BI, Bz E f?. By M2, any masking quorum system for B must contain quorums Q; ~ Q,, Q~ G Qz. However, for any such Q~, Q:, it is the case that (Q{ nQ:)\B’ ~ (Ql nQa)\B’ g B“, violating Ml. Therefore, there does not exist a masking quorum system for B under the assumption that Q is not a masking quorum system for B. ❑
Corollary 4.4 Let B be a fail-prone system for a universe U. Then there exists a masking quorum system for B iff for d B1,BZ,BS,B4 ~ B,U ~ B1UBZUB3UB4. hp=ticd~, suppose that B = {B G U : IBI = j}. Then, there exists a masking quorum system for B iff n > 4f.
Proof. By Theorem 4.3, there is a masking quorum for B iff Q = {U \ B : B E B} is a masking quorum for B. By construction, Q is a masking quorum iff M 1 holds for Q, i.e.,
Proof. Let w be any strategy for the quorum system Q, and fix Q1 E Q such that [QI I = C(Q). Summing the loads induced by w on all the elements of Q1 we obtain:
Q;
=1
Therefore, there exists some element in QI that stiers a
10a‘dfalteMat“
Similarly, summing the total load induced by w on all of
the elements of the universe, we get:
ifffOr& B1,B1,BS,B4
EB:
Bl)~(u\B2))\&g B4
B2)~&ui?4
B2u B3u B4.
(Here, the inequality results from the minimality of c(Q).) Therefore, there exists some element in U that sfiere a load
of at least *. ❑
Since any masking quorum system is a quorum system, we
have, a fortiorfi
Corollary 4.6 If Q is a masking quorum system over a
@} ~d universe of n elements, then L(Q) > max{ ~, ~
thus L(Q) > ~.
Below we give several examples of masking quorum sys- tems and describe their properties.
Example 4.7 (Threshold) Suppose that B = {B ~ U :
IBI = f}, n > 4f. Note that this corresponds to the usual
threshold assumption that up to ~ servers may fail. Then,
the quorum system Q = {Q ~ U : IQI = [~1} is
a masking quorum system for B. M1 is satisfied because
~Y Q1~QZ E Q wi~ intersect in at l==t zf + 1 dernents. M2 holds because [~ 1 ~ n – ~. A strategy that as-
signs equal probability to each quorum induces a load of
1 on the system. By Corollary 4.6, this load is in fact the load of the system. ❑
The following example is interesting since its load de- creases as a function of n, and since it demonstrates a method for ensuring system-wide consistency in the face of Byzan- tine failures while requiring the involvement of fewer than a majority of the correct servers.
Example 4.8 (Grid quorums) Suppose that the universe of servers is of size n = kz for some integer k and that B = {B ~ U : IBI = f}, 3j+ 1< X. Arrange the universe into a W x W grid, as shown in F@re 1. Denote the rows
E!
((u\ - u\(&u
* ugB1u
The following theorem was proved in [30] for benign- failure quorum systems, and holds for masking quorums as well (as a result of Ml). Let c(Q) denote the size of the smallest quorum of Q.
`Theorem 4.5` If Q is a quorum system over a universe of n &l}. elements, then L(Q) ~ max{ -&j, .
The proof of this theorem in [30] employs rather complex methods. Here we present a simpler proof of their theorem.
572
:[+

 and columns of the grid by R; and C;, respectively, where

##### 5 Variations

##### 5.1 Dissemination quorum systems

As a special case of services that can employ quorums in a Byzantine environment, we now consider applications in which the service is a repository for self verifying information, i.e., information that only clients can create and to which clients can detect any attempted modification by a faulty server. A natural example is a database of public key certificates as found in many public key distribution systems (e.g., [10, 37, 23]). A public key certificate is a structure con- taining a name for a user and a public key, and represents the assertion that the indicated public key can be used to au- thenticate messages from the indicated user. This structure is digitally signed (e.g., [35]) by a certification authority so that anyone with the public key of this authority can verify this assertion and, providing it trusts the authority, use the indicated public key to authenticate the indicated user. Due to this signature, it is not possible for a faulty server to undetectable modify a certificate it stores. However, a faulty server can undetectable suppress a change from propagating to clients, simply by ignoring an update from a certification authority. This could have the effect, e.g., of suppressing the revocation of a key that has been compromised.

As can be expected, the use of digital signatures to verify data improves the cost of accessing replicated data. To support such a service, we employ a dissemination quorum system, which has weaker requirements than masking quo- rums, but which nevertheless ensures that in applications like those above, self verifying writes will be propagated to all subsequent read operations despite the arbitrary failure of some servers. To achieve this, it suffices for the inter- section of every two quorums to not be contained in any set of potentially faulty servers (so that a written value can propagate to a read). And, supposing that operations are required to continue in the face of failures, there should be quorums that a faulty set cannot disable.

Definition 5.1 A quorum system Q is a dissemination 

```
guo-
1< z < X. Q=
{
Then, the quorum system
cjuu
Ri:~,{j}C{l... fi},l~l=2f+ 11
icI
```

is a masking quorum system for B. M 1 holds since every pair of quorums intersect in at least 2f + 1 elements (the column of one quorum intersects the 2~ + 1 rows of the other), and M2 holds since for any choice of ~ faulty elements in the grid, 2f + 1 full rows and a column remain available. A strategy that assigns equal probabdit y to each quorum induces a load of (2f+2)fi-(2f+l
~, and again by Corollary 4.6, this is the load of the”system. ❑
Note that by choosing B = {O} (i.e., f = O) in the exam- ple above, the resulting construction has a load of 0( $=),
which asymptotically meets the bounds given in Corollary 4.6. In general, however, this construction yields a load of 0(~), which is not optimal: Malkhi et al. [28] show a lower
bound of ~ ~ on the load of any masking quorum sYs- r
tern for B = {B ~ U : lB/ = f}, and provide a construction whose load matches that bound.
Figure 1: Grid construction, k.x k = n, ~ = 1 (one quorum shaded).
Example 4.9 (Partition) Suppose that B = {131,..., B-},
m > 4, is a partition of U where Bi # 0 for all i, 1 < i < m. This choice of B could arise, for example, in a wide area network composed of multiple local clusters, each containing some Bi, and expresses the assumption that at any time, at most one cluster is faulty. Then, any collection of nonempty sets B1 ~ E1, 1 ~ i < m, can be thought of as ‘super-elements’ in a universe of ~lze m, with a threshold assumption ~ = 1. Therefore, the following is a masking quorum system for B:
rurn sys tern for a fail-prone erties are satisfied.
Dl:vQ,, QzEQVBEB:
D2:VBEB3QEQ:
❑
system B if the following prop- QlnQ~qB
Q= uEi { i~I
: IC{l,...,
m}l,Il=(~l 1

A dissemination quorum system will suffice for propagat- ing self-verifying information as in the application described above. The write operation is implemented as described in Section 3, and the read operation becomes:

Read: For a client to read a variable z, it queries each server in some quorum Q to obtain a set of value/timestamp pairs A = {<vu, tu>}uEQ. The client then discards those pairs that are not verifiable (e.g., using an appropriate e digital signature verification algorithm) and chooses from the re- maining pairs the pair <u,t>with the largest timestamp. u is the result of the read operation.

It is important to note that timestamps must be included as part of the self-verifying information, so they cannot be undet ect ably altered by faulty servers. In the case of the ap- plication described above, existing standards for public key certificates (e.g., [10]) already require a real-time timestamp in the certificate.

Ml is satisfied because the intersection of any two quorums contains elements from at least three sets in B. M2 holds since thereis no B E B thatintersects all quorums. A strat- egy that assigns equal probability to each quorum induces a load of ~ ~WI on the system regardless of the size of each
~i, and again Corollary 4.6 implies that this is the load of the system.

If m = k2 for some k. then a more efficient construction can be achieved by forming the grid construction from Ex- ample 4.8 on the ‘super elements’ {~i }, achieving a load of *.m ❑
573
BnQ=O

 The following lemma proves correctness of the above pro- tocol using dissemination quorum systems. The proof is al- most identical to that for masking quorum systems.

Lemma 5.2 A read operation that is concurrent with no write operations returns the value written by the last pre- ceding write operation in some serialization of all preceding write operations.
Due to the assumption of self-verifying data, we can also prove in this case the following property.
Lemma 5.3 A read operation that is concurrent with one or more write operations returns either the value written by the last preceding write operation in some serialization of all preceding write operations, or any of the values being written in the concurrent write operations.

The above lemmata imply that the protocol above im- plements a single-writer single-reader regular variable [24]. Theorems analogous to the ones given for masking quorum systems above are easily derived for disseminateion quorums. Below, we list these results without proof.

`Theorem 5.4` Let B be a fail-prone system for a universe U. Then there exists a dissemination quorum system for B iff Q = {U \ 13 : B E B} is a dissemination quorum system for B.

`5.2` Opaque masking quorum systems
Masking quorums impose a requirement that clients know the fail-prone system t?, while there may be reasons that clients should not be required to know this. First, it some- what complicates the client’s read protocol. Second, by re- vealing the failure scenarios for which the system was de- signed, the system also reveals the failure scenarios to which it is vulnerable, which could be exploited by an attacker to guide an active attack against the system. By not reveal- ing the fail-prone system to clients, and indeed giving each client only a small fraction of the possible quorums, the sys- tem can somewhat obscure (though perhaps not secure in any formal sense) the failure scenarios to which it is vulner- able, especially in the absence of client collusion.
In this section we describe one way to modify the mask- ing quorum definition of Section 4 to be opaque, i.e., to elim- inate the need for clients to know 1?. In the absence of the client knowing B, the only method of which we are aware for the client to reduce a set of replies from servers to a single reply from the service is via voting, i.e., choosing the reply that occurs most often. In order for this reply to be the cor- rect one, however, we must strengthen the requirements on our quorum systems. Specifically, suppose that the variable z is written with quorum Q1, and that subsequently x is read with quorum Q2. If 1? is the set of arbitrarily faulty servers, then (Ql n Q2 ) \ B is the set of correct servers that possess the latest value for z (see Figure 2). In order for the client to obtain this value by vote, this set must be larger than the set of faulty servers that are allowed to respond, i.e., Q2 n B. Moreover, since these faulty servers can “team up” with the out-of-date but correct servers in an effort to suppress the write operation, the number of correct, up-to-date servers that reply must be no less than the number of faulty or out-of-date servers that can reply, i.e., (Qz fl B) U (Qz \ Q1 ).

`Definition 5.10` A quorum system Q is an opaque masking quorum system for a fail-prone system B if the following properties are satisfied.
01: VQI, Q2EQVBEZ3:
l(Qln QZ)\Bl z l(Qzn B) U(Qz\Ql)l
Corollary
U. Then there exists a dissemination quorum system for B iff for all B1,B2,Bs E B, U ~ B1 Ul?Z uB3. Inparticukm, suppose that B = {B ~ U :IBI = f}. Then, there exists a dissemination quorum system for B iff n > 3f.
Corollary 5.6 If Q is a dissemination quorum system over
Q}, a universe of n elements, then L(Q) z max{ ~, .
and thus also L(Q) ~ -&.
Below, we provide several example constructions of dissem- inateion quorum systems.
Example 5.7 (Threshold) Suppose that B = {B Q U : IBl = f}, n > 3f. Note that this corresponds to the usual threshold assumption that up to f servers may fail. Then, the quorum system Q = {Q G U : IQI = [~1} is a
dissemination quorum system for B with load ~ [-l. •l
Example 5.I3 ( Grid) Let the universe be arranged in a grid as in Example 4.8 above, and let B = {B G U : IBI = f}, 2j + I < W. Then, the quorum system
5.5 Let B be a fail-prone system for a universe
Q= cjuu Ri:I, {j}~{l...fi},lI[=f+l { ieI
}
B
Q2
01:
02:

is a dissemination quorum system for B. The load of this system is “+a)~-(’+l]. ❑
Example 5.9 (Partition) Suppose that B = {Bl, . . . . B~}, m > ~, is a partition of U. For any collection of nonempt y sets B; ~ B;, 1 < i < m, the Threshold construction of Example 5.7 on the ‘super-elements’ Bi ~ Bi (as in Example 4.9) yields a dissemination quorum system with a load of ~ [*1. If m = k’ for some k, the Grid construction of
Example 5.8 achieves a load of %. ❑
Note that 01 admits the possibility of equality in size be- tween (Q1 n Qz) \B and (Q2 fl II) U (Qa \ Ql). Equality is
574
❑
02: VQ,, Q2EQVBEB:
l(Q~nQz)\Bl>lQ,f7Bl
BnQ=O
03, vB~B3Qc
Q:

 sufficient since, in the case that the faulty servers “t earn up” with the correct but out-of-date servers in QZ, the value re- turned from (QI fI Q2 ) \ El will have a higher timestamp than that returned by (Qz n B) u (Qz \ Q1). Therefore, in the case of a tie, a reader can choose the value with the higher times- tamp. It is intcresting to note that a strong inequality in 01 would permit a correct implementation of a single-reader singer-writer safe variable that does not use timestamps (by taking the majority value in a read operation).

It is not difficult to verify that an opaque masking quo- rum system enables a client to obtain the correct answer from the service. The write operation is implemented as de- scribed in Section 1, and the read operation becomes:

Read: For a client to read a variable z, it queries each server in some quorum Q to obtain a set of value/timestamp pairs A = {<vU, tw>}.~Q. The client chooses the pair <u, t> that appears most often in A, and if there are multiple such val-
Example 5.14 (Partition) Suppose that B = {Bl, . . . . Bs~}, k>1, isapartitionofUwhereBi#0foralli,1< i<3k. Choose any collection of sets fii ~ B;, 1< i< 3k, such that l~i I = c for a fixed constant c >0. Then, the Threshold construction of Example 5.12 on the ‘super-elements’ {A;} (as in Example 4.9), with universe size 3k and a threshold assumption ~ = 1, yields an opaque quorum system with load ~. ❑

Unlike the case for regular masking quorum systems, an open problem is to find a technique for testing whether, given a fail-prone system 1?, there exists an opaque quorum system for 23(other than an exhaustive search of all subsets of 2~).

In the constructions in Examples 5.12 and 5.13, the re- sulting quorum systems exhibited loads that at best were constant as a function of n. In the case of masking quorum systems, we were able to exhibit quorum systems whose load decreased as a function of n, namely the grid quorums. A natural question is whether there exists an opaque quorum system for any fail-prone system B that has load that de- creases as a fhnction of n. In this section, we answer this question in the negative: we show a lower bound of ~ on the load for any opaque quorum system construction, regardless of the fail-prone system.
ues, the one with the highest timestamp. result of the read operation.

The value v is the
Opaque masking quorum systems, combined with the access protocol described previously, provide the same semantics as regular masking quorum systems. The proof is ahnost identical to that for regular masking quorums.

Lemma 5.11 A read operation that is concurrent with no write operations returns the value written by the last preceding write operation in some serialization of all preceding write operations.

Below we give several examples of opaque masking quo- rum systems (or just “opaque quorum systems”) and de- scribe their properties.

`Example 5.12` (Threshold) Suppose that B = {B ~ U :
Theorem at least *.
5.15 The load of any opaque quorum system is
IBI = }} where n z 5j and ~ >0. system Q = {Q ~ U : IQI= [~]} system for B, whose load is ~ [~
Then, the quorum is an opaque quorum
Proof. 01 implies that for any Q1, Q2 E Q, IQ1 fl QzI >
IQ1\Qzl, ~d th~ IQ1n Qzl z ~. Let w be any strategy for the quorum system Q, and flx any QI E Q. Then, the total load induced by w on the elements of Q1 is:
.IQJ1 2
Therefore, there must be some server in Q1 that sufTers a load at least ~. ❑
We now present a generic construction of an opaque quo- rum system for B = {0} and increasingly large universe sizes n, that has a load that tends to ~ as n grows. We give this construction primarily to show that the lower bound of ~ is tight; due to the requirement that B = {0}, this con- struction is not of practical use for coping with Byzantine failures.
Example 5.16 Suppose that the universe of servers is U =
{u,,..., u.} where n = 24 for some .f > 2, and that B =
{0}. Consider the n x n Hadamard matrix H(t), constmcted ..
1. ❑
The next theorem proves a resilience bound for opaque quo-
rum systems.
Theorem 5,13 Suppose that B = {B ~ U : IBI = f}. There exists an opaque quorum system for B iff n z 5f.
Proof. That n ~ 5f is sufficient is already demonstrated in Example 5.12 above. Now suppose that Q is an opaque quorum system for B. Fm any Q1 E Q SUChthat IQ1[ < n–f (QI exists by 03); note that IQ, I > ~ by 02. Choose B1 ~ Q,, IBII = f, and some Q, E Qsuch that Q, C U\Bl (Qz exists by 03). Then IQ1n Qzl < n – 2~. By 02,
IQ1nQZI2 f, =d therefore there is some B, E B such that Bz G Q1 n Qz. Then
n–3~ z
= l(Q~nQl)\Bal
> I(Q1 \Q2)u(Ql = lQl\Qzl+lBzl ~ IB11+IB21
= 2f
lQ~nQ1l– IBz I
Where (1) holds by 01. Therefore, we have n ~ 5f. ❑
nBz)l (1)
575
recursively as follows:
H(l) = _l ~
H(k) =
[
H(k – 1) H(k – 1)
H(k – 1) –H(k – 1)
[1
–1 –1

 H(t) has the property that HAT = nl, where Z is the n x n identity matrix. Using well-known inductive ar- guments [16, Ch. 14], it can be shown that (i) the fist row and column consist entirely of – 1‘s, (ii) the i-th row and i-th column, for each i z 2, has 1’s in ~ positions (and simi- larly for –l’s), and (iii) any two rows (and any two columns) i, j z 2 have identical elements in ~ positions, i.e., 1’s in ~ common positions and —1‘s in ~ common positions.
We treat the rows of H(t) as indicators of subsets of U. That is, let Qi = {uj : H(t)[i, j] = 1} be the set defined bythei-throw,1<i<n. NotethatQ1=0andthat UI is not included ~ any Qi. We claim that the system Q = {Qz, ....Q~} is m opaque quorum system for B. Using properties (i)–(iii) above, we have that IQ; I = ~ for each i z 2; that each ui, i ~ 2, is in exactly ~ of the sets Qa, ...,Q~; andthatforanyt,~z2,ifi#~thenlQin Qjl=f. From these, the 01 and 02 requirements can be quickly verified,
and a load of & can be achieved, e.g., with a strategy that assigns equal probability to each quorum. ❑

### 6 Faulty clients

So far, we have been concerned with providing a consistent service to a set of correct clients. In this section, we extend our treatment to address faulty clients in addition to faulty servers, as would be required if servers are allowed to act as (or on behalf of) clients. Since updates may now be generated by faulty clients, we can make no assumption of self-verifying data, and thus use masking quorum systems (Section 4) to implement the service. We focus on ensuring the consistency of the data stored at the replicated service as seen by correct clients only. Since a faulty client might not complete a write operation at a quorum of servers, or might even write different values to different servers, in this section we modify the write protocol to include an update protocol implemented by the servers that prevents clients from leav- ing the service in an inconsistent state. This update protocol could be implemented using well-known agreement protocols (e.g., [22, 8]), but only if the given fail-prone system B has the property that each B G B is of size less than lU\/3, and only by involving all of the servers in the system. We de- scribe a protocol that is correct for any fail-prone system B for which a masking quorum exists, and that involves only a quorum of correct servers to complete an update oPera- tion. While we do not explicitly treat load in this section, this latter property is essential for the load measure that we have adopted to be usefti.

### 6.1 The write protocol
This section describes the protocol by which clients write the variable z replicated at each server. We replace the write operation of Section 3 by the following procedure:

`Write:` For a client c to write the value u, itqueries each server in some quorum Q to obtain a set of value/timestamp pairs A = {cu., t.>}.cQ; chooses a timestamp t E T=greater than the highest timestamp value in A and greater than any timestamp it has chosen in the paat; and performs Init(Q, v, t).
Note that writing the pair <u,t> to the quorum Q is performed by executing the operation lnit(Q, u, t).Servers ex- ecute corresponding events ~eliver(u, t).If a correct server executes Deli ver(u, t),and if t is greater than the timest amp currently stored with the variable, then the server updates
the value of the variable and its timestamp to v and t, re- spectively. Regardless of whether it updates the variable, it sends an acknowledgment message to c where Tc 3 t.
The correctness of this protocol depends on the following relationships among hit executions at clients and Deliver events at servers. How to implement kit and DeJiver to satisfy these relationships is the tepic of Section 6.2.
Integrity: If c is correct, then a correct server executes Deliver(u, t)where t ~ T. only if c executed lnit(Q, u,t)for some Q E Q.
Agreement: If a correct server executes Deliver(u, t)and a correct server executes Deliver(u’, t), then w = u’.

`Propagation:` If a correct server executes Deliver(u, t), then eventually there exists a quorum Q E Q such that every correct server in Q executes Deliver(u, t).
Validity: If a correct client executes Init(Q, v,t)and all servers in Q are correct, then eventually a correct server ex- ecutes Deli ver(v, t).

Note that by Validity, if a correct client executes lnit(Q, v, t) but Q contains a faulty server, then there is no guarantee that Deliver (u,t)will occur at any correct server; i.e., the write operation may have no effect. A correct server ac- knowledges each Deliver (v, t) execution as described above to inform the client that Deliver (o, t) was indeed executed. If the client receives acknowledgments from a set B+ of servers, such that VB ~ B : B+ ~ B, then it is certain that its write will be applied at all correct servers in some quorum Q (by Propagation). If the client receives acknowledgments from no such set B+ of servers, then it must attempt the Init operation again with a different quorum. As before, M2 guarantees the availability of some quomm.

In order to argue correctness for this protocol, we have to adapt the definition of operation precedence to allow for the behavior of a faulty client. The reason is that it is unclear how to define when an operation by a faulty client begins or ends, as the client can behave outside the specification of LWIYprotocol. We now say that a write operation that writes v with timestamp tE T., where c is faulty, begins when the first correct server executes Deliver(v, t) and ends when all correct servers in some quorum have executed Deli ver(v, t). Write operations by correct clients begin as before and end when all the correct servers in some quomm have delivered the update. We do not define or make use of the duration of a read by a faulty client; reads by faulty clients are not ordered with respect to other operations. Carrying over the remainder of the precedence definition, a proof very similar
to that of Lemma 4.2 suffices to prove the following:
Lemma 6.1 A correct process’ read operation that is con- current with no write operations returns the value written by the preceding write operation with the highest timestamp among all preceding write operations.

We are not aware of any common definition of variable semantics in the case of possibly faulty clients with whit% to compare Lemma 6.1. However, note that if all the write operations preceding the read are done by correct clients, the highest timestamp value among them will belong to the last write in some serialization of them, and therefore the read will return that value.
576

### 6.2 The update operation

The remaining protocol to describe is the update protocol for masking quorum systems that satisfies Integrity, Agree- ment, Propagation, and Validity. We present such an update protocol in F@re 3.

1. If a client executes lnit(Q, v, t),then it sends <update, Q, v, t> to each member of Q.

2. If a server receives <update, Q, v, t> from a client c, if tE Tc, and if the server has not previously received from c a message <update, Q’, u’, t’>where either t’ = t and V’ # v or t’ > t, then the server sends <echo, Q, V, t> to each member of Q.

3. If a server receives identical echo messages <echo, Q, w, t> from every server in Q, then it sends <ready, Q, v, t> to each member of Q.

4. If a server receives identical ready messages <ready, Q, v, t> from a set B+ of servers, such that B+ ~ B for all B E B, then it sends <ready, Q, v, t> to every member of Q if it has not done so already.

5. If a server receives identical ready messages <ready, Q, u, t>from a set Q– of servers, such that for some B ~ B, Q- = Q \B, it executes Deliver(v, t).
F@re 3: An update protocol
Lemma 6.2( Integrity) If c is correct, then a correct server executes Ile]i ver (v, t) where t E Z’c ordy if c executed lnit(Q, u, t) for some Q.

`Proof.` The first <ready, Q, v, t>message from a correct server is sent only after it receives <echo, Q, v, t>from each member of Q. Moreover, a correct member sends <echo, Q, v, t>where t E Tc only if it receives <update, Q, u, t> from c over an authenticated channel, i.e., only if c executed lnit(Q, u, t). ❑
Lemma 6.3( Agreement) If a correct server executes Deliver(v, t)and a correct server executes Deliver(v’, t), then v = v’.
Proof. As argued in the previous lemma, for a correct server to execute Ileliver(u, t), <echo, Q, u, t>must have been sent by all servers in Q. Similarly, <echo, Q’, v’, t>must have been sent by all servers in Q’. Since every two quorums intersect in (at least) one correct member, and since any correct server sends <echo, *, fi, t>for at most one value 0, v must be identical to v’. ❑

`Lemma 6.4` If Q is a masking quorum system over a universe U with respect to a fail-prone system B, then VQ c QVB,, B,, B,~B, Q~B, uB, uB,.
Proof. Assume otherwise for a contradiction, i.e., that there isa Q6Qand Bl, Bl, B9~f3 such that Q~BIUBIUBs.
By M2, there exists Q’ E Q, Q’ n B, = 0. Then, Q n Q’ c Ba U Bs and thus (Q n Q’) \Ba ~ Bs, contradicting Ml. ❑

`Lemma 6.5`( Propagation) If a correct server executes Deliver (v, t), then eventually there exists a quorum Q E Q such that every correct server in Q executes Deli ver(v, t).
Proof According to the protocol, the correct server that executed Deli ver(v, t) received a message <ready, Q, u, t> from each server in Q- =Q\Bforsome Q~Qand B~B. Since, for some B’ E B, (at least) all the members in Q-\ B’ are correct, every correct member of Q receives <ready, Q, v, t>from each of the members of B+ = Q– \ B’. Since, VB” E B, Q– \B’ ~ B“ (by Lemma 6.4), the ready messages from B+ cause each correct member of Q to send such a ready message. Consequently, Deliver(v, t) is executed by all of the correct members of Q. ❑
Lemma 6.6( Validity) If a correct client c executes lnit(Q, v, t) and all servers in Q are correct, then eventually a correct server executes Deli ver(u, t).
Proof. Since both the client and all of the members of Q are correct, <update, Q, v, t> will be received and echoed by every member in Q. Consequently, all the servers in Q will send <ready, Q, v, t>messages to the members of Q, and will eventually execute Deliver (v, t). •I

### 7 Conclusions

The literature contains an abundance of protocols that use quorums for accessing replicated data. This approach is ap- pealing for constructing replicated services as it allows for increasing the availability and efficiency of the service while maintaining its consistency. Our work extends this success- ful approach to environments where both the servers and the clients of a service may deviate from their prescribed behavior in arbitrary ways. We introduced a new class of quorum systems, namely masking quorum systems, and devised pro- tocols that use these quorums to enhance the availability of systems prone to Byzantine failures. We also explored two variations of our quorum syst ems, namely dissemination and opaque masking quorums, and for all of these classes of quo- rums we provided various constructions and analyzed the load they impose on the system.
Our work leaves a number of intriguing open challenges and directions for future work. One is to characterize the average performance of our quorum constructions and their load in less-than-ideal scenarios, e.g., when failures occur. Also, in this work we described only quorum systems that are uniform, in the sense that any quorum is possible for both read and write operations. In practice it may be ben- eficial to employ quorum systems with distinguished read quorums and write quorums, with consistency requirements imposed only between pairs consisting of at least one write quorum. Although this does not seem to improve our lower bounds on the overall load that can be achieved, it may al- low greater flexibility in trading between the availability of reads and writes.
Acknowledgments

We are grateful to Andrew Odlyzko for suggesting the use of Hadamard matrices to construct opaque masking quo- rum systems with an asymptotic load of ~. We also thank Yehuda Afek and Mk.hael Merritt for he1pful discussions, and Rebecca Wright for many helpful comments on earlier versions of this paper. An insightful comment by Rida Bazzi led to a substantial improvement over a previous version of this paper.
577

<!--


 References
[1] Y. Afck, H. Attiya, D. Dolev, E. Gafni, M. Merritt and N. Shavit. Atomic snapshots of shared memory. Journal of the ACM 40(4):873-890, September 1993.
[2] Y. Afek, D. Dolev, E. Gafni, M. Merritt and N. Shavit. A bounded first-in first-enabled-solution to the l-exclusion problem. In Proceedings of the ~th International Workshop on Distributed Algorithrrq LNCS 486, Springer-Verlag, 1990.
[3] D. Agrawd and A. El Abbadi. Integrating security with fault-tolerant distributed databases. Computer Journal 33(1):71–78, February 1990.
[4] D. Agrawal and A. El Abbadi. An efficient and fault- tolerant solution for distributed mutual exclusion. ACM
I%znsactions on Computer systems 9(1):1-20, 1991.
{5]J. H. Anderson. Composite registers. Distributed Cornpuf-
ing 6(3):141-154, 1993.
[6] H. Attiym, A. Bar-Noy and D. Dolev. Sharing Memory Ro-
[23] B. Lampson, M. Abadi, M. Burrows, and E. Wobber. Au- thentication in distributed systems: Theory and practice. ACM Transactions on Computer Systems 10(4):265–310, November 1992.
[24] L. Lamport. On interprocess communication (part II: algo- rithms). Distributed Computing 1 :86–101, 1986.
[25] M. Li, J. Tromp and P. M. B. Vitanyi. How to share con- current wait-free variables. Journal of the A CM, to appear. [26] M. Maekswa. A W algorithm for mutual exclusion in de-
centralized systems. ACM Tmnsactions on Computer Sys-
temrn3(2):145-159, 1985.
[27] D. Malkhi and M. Rciter. A high-throughput secure reli-
able multicast protocol. Journal of Computer Security, to appear. Also in Proceedings of the 9th IEEE Computer ,Se- cus-ity Foundation Workshop, pages 9–17, June 1996.
[28] D. Malkhi, M. Reiter and A. Wool. Optimal Byzantine quo- rum systems. Submitted for publication, January 1997.
[29] R. Mukksmala. Storage efficient and secure replicated dis- tributed databases. IEEE Transactions on Knowledge and Data Engineering 6(2):337-341, April 1994.
[30] M. Naor and A. WOO1.Tbe load, capacity, and availability of quorum systems. In Proceeding~ of the 95th IEEE Sympo- sium on Foundations of Computer Science, pages 214–225, 1994.
[31] M. Naor and A. Wool. Access control and signatures via quorum secret sharing. In Proceedings of the Srd ACM Con- ference on Computer and Communications Security, pages 157–168, March 1996.
[32] M. O. Rabin. Efficient dispersal of information for security, load balancing, and fault tolerance. Journal of the ACM 36(2):335-348, 1989.
[33] M. K. Reiter. Secure agreement protocols: Reliable and atomic group multicast in Rampart. In Proceedings of the %d ACM Conference on Computer and Communication Security, pages 68–8o, November 1994.
[7]
[B]
[9]
[IO]
[1 I]
[12]
[13]
[14]
(15]
[16] [17]
[18]
[19]
[20]
[21]
[22]
bustly in Message-Passing Systems. Journal of the ACM 42(1):124–142, January 1995.
P. A. Bernstein, V. Hadzilacos and N. Goodman. Concur- rency control and recovery in databaae syaterns. Addiscm- wesley, 1987.
G. Bmcha and S. Toueg. Asynchronous consensus and brosdcast protocols. Joumai of the ACM 32(4):824–840, October 1985.
S. Y. Cheung, M. H. Ammar, and M. Ahamad. The grid protocol: A high performance scheme for maintaining repli- cated data. In Proceedings of the 6th IEEE International Conference on Data Engineering, pages 438-445, 1990. International Telegraph and Telephone Consultative Com- mittee (CCITT). The Directory – Authentication Frame- work, Recommendation X .5o9, 1988.
D. Dolev, E. Gafni and N. Shavit. Toward a non-atomic era: l-exclusion as a test case. In Pmceedinga of the .?Oth ACM Symposium on Theory of Computing, pages 78–92, May 1988.
D. Dolcv and N. Shavit. Bounded concurrent time-stamp systems are constructible. SIAM Journal of Computing, to appear. Also in Proceedings of the 21st ACM Symposium on the Theory of Computing, pages 454–466, 1989.
A. El Abbadi and S. Toueg. Maintaining availability in partitioned replicated databases. ACM Tmnsactiom on Databaee Syatema 14(2):264-290, June 1989.
H. Garcia-MoIina and D. Barbara. How to assign votes in a distributed system. Journal of the ACM 32(4):841–860, October 1985.
D. K. GiEord. Weighted voting for replicated data. In Pru- ceedinga of the 7th ACM Symposium on Opemting Systems Principles, pages 150-162, 1979.
M. Hall, Jr. Combinatorial Theory. 2nd Ed. Wiley- Interscience Series in Discrete Mathematics, 1986.
M. Herlihy. A quorum-consensus replication method for rab- stract data types. ACM Tmnaactiona on Computer Syatema 4(1):32–53, February 1986.
M. P. Herlihy and J. D. Tygar. How to make replicated data sccurc. In Admzncee in Cryptology— CRYP TO ’87 Procee- dings(Lecture Notes in Computer Science 293), pages 379– 391, Springer-Verlag, 1988.
A. Israeli and M. Li. Bounded time-stamps. Distributed Computing 6(4):205-209.
A. Kumar. Hierarchical quorum consensus: A new algo- rithm for managing replicated data. IEEE Transactions on
Computers 40(9):996-1004, 1991.
A. Israeli and A. Shaham. Optimal multi-write multi-reader atomit register. In Proceedings of the 11th ACM Sympo- sium on Principles of Distributed Computing, pages 71–82, 1992.
L. Lamport, R. Shostak and M. Pease. The Byzantine gen- erals problem. ACM Transactions on Programming Lan- guages and Systems 4(3):382-401, July 1982.
[34] [35]
[36]
[37]
[38]
[39]
M. K. Reiter. Distributing trust with the Rampart toolkit. Communications of the ACM 39(4) :71-74, April 1996.
R. Rivest, A. Shamir, and L. Adleman. A method for obtaining digital signatures and public-key cryptosystems.
Communications of the ACM 21 (2):120-126, February 1978.
A. Shamir. How to share a secret. Communications of the ACM 22(11):612-613, November 1979.
J. J. Tardo and K. Alagappan. SPX: Global authentication using public key certificates. In Proceedings of the 1991 IEEE Symposium on Research in Security and Privacy, pages 232–244, May 1991.
R. H. Thomas. A majority consensus approach to concur- rency control for multiple copy databases. ACM Tmnaac- tions on Database Systems 4(2):180–209, 1979.
S. Toueg. Randomized Byzantine agreement. In Proceedings of the 3rd ACM Symposium on Principles of Distributed
Computing, pages 163–1 78, August 1984.
578


<!--
Byzantine Quorum Systems Dahlia Malkhi Michael Reiter
AT&T Labs—Research, Murray Hill, NJ USA
{dalia,reiter}@research.
att .com
tence of masking quorum systems under different failure assumptions. -->

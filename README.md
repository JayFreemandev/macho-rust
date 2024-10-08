# 러스트 스터디

* 자료: ["러스트 DOCS"]( https://doc.rust-lang.org/stable/book/ )
* 참여자: @jayfreemandev 혼자

이 스터디의 특징은 다음과 같다.

* 모든 코드를 직접 입력해본다.
* 경우에 따라 코드를 그대로 사용하지 않고 약간의 변형을 가한다.
* 하는 김에 BDD 테스트 코드도 함께 작성한다.
* 단계별로 git commit 을 하여, 읽으면서 순서대로 따라올 수 있도록 한다.

## 목차

* [챕터 01. 시작]( #챕터-01-시작 )
* [챕터 02. 추리게임]( #챕터-02-추리게임 )
* [챕터 03. 일반적 개념]( #챕터-03-일반적-개념 )
* [챕터 04. 소유권 이해]( #챕터-04-소유권-이해 )
* [챕터 05. 데이터 구조화하기]( #챕터-05-데이터-구조화하기 )
* [챕터 06. 열거형과 패턴매칭]( #챕터-06-열거형과-패턴매칭 )
* [챕터 07. 커져가는 프로젝트 관리]( #챕터-07-커져가는-프로젝트-관리 )
* [챕터 08. 컬렉션]( #챕터-08-컬렉션 )
* [챕터 09. 에러 처리]( #챕터-09-에러-처리 )
* [챕터 10. 제너릭 타입, 트레이트, 라이프타임]( #챕터-10-제너릭-타입-트레이트-라이프타임 )
* [챕터 11. 자동화 테스트]( #챕터-11-자동화-테스트 )
* [챕터 12. I/O 프로젝트 커맨드 프로그램]( #챕터-12-I/O-프로젝트-커맨드-프로그램 )
* [챕터 13. 반복자와 클로저]( #챕터-13-반복자와-클로저 )
* [챕터 14. 스마트 포인터]( #챕터-14-스마트-포인터 )
* [챕터 15. 동시성]( #챕터-15-동시성 )
* [챕터 16. OOP]( #챕터-16-OOP )
* [챕터 17. 패턴과 매칭]( #챕터-17-패턴과-매칭 )
* [챕터 18. 고급 기능]( #챕터-18-고급-기능 )
* [챕터 19. 멀티스레드 웹서버 프로젝트]( #챕터-19-멀티스레드-웹서버-프로젝트 )

## 챕터 01. 시작

* [학습 코드]( https://github.com/johngrib/study-objects/pull/1 )

![image](https://user-images.githubusercontent.com/1855714/75627164-429ce780-5c11-11ea-8fc8-51a365d08f82.png)

* [테스트 코드]( https://github.com/johngrib/study-objects/blob/e3ad439bf0263f0b5c1056167020e3a1a31397ff/src/test/java/com/johngrib/objects/_01_ticket/AudienceTest.java )

![image](https://user-images.githubusercontent.com/1855714/75622897-7dd6f080-5be8-11ea-91e1-c900fbac8361.png)

## 챕터 02. 추리게임

* [학습 코드]( https://github.com/johngrib/study-objects/pull/2 )

![image](https://user-images.githubusercontent.com/1855714/75777683-b9fd8300-5d99-11ea-8e98-77092fedd37e.png)

* [테스트 코드]( https://github.com/johngrib/study-objects/blob/3fbf01fdc801663ed07f9436ddafb09c6db63557/src/test/java/com/johngrib/objects/_02_movie/MovieTest.java )

![image](https://user-images.githubusercontent.com/1855714/75777291-0f856000-5d99-11ea-9b62-7a4c84160f0c.png)

## 챕터 03. 일반적 개념

이 챕터에는 코드 예제가 없다.
<!-- 
## 챕터 04. 소유권 이해

* [학습 코드]( https://github.com/johngrib/study-objects/pull/3 )

![image](https://user-images.githubusercontent.com/1855714/76524750-f6be2e00-64ad-11ea-84b3-a799950834da.png)

* [테스트 코드]( https://github.com/johngrib/study-objects/tree/c05c319968cb52f8109e0d3c985de3c9b20769aa/src/test/java/com/johngrib/objects/_04_movie_data_system )

![image](https://user-images.githubusercontent.com/1855714/76524433-697ad980-64ad-11ea-9e99-e3dff162d0d8.png)

## 챕터 05. 데이터 구조화하기

* [학습 코드]( https://github.com/johngrib/study-objects/pull/4 )

![image](https://user-images.githubusercontent.com/1855714/77227293-940e1600-6bc2-11ea-9e40-ce378804cadd.png)

## 챕터 06. 열거형과 패턴매칭

* [학습 코드]( https://github.com/johngrib/study-objects/pull/6 )

![image]( https://user-images.githubusercontent.com/1855714/77823475-c8dc1900-713e-11ea-969b-a3557af77905.png )

## 챕터 07. 커져가는 프로젝트 관리

* [학습 코드]( https://github.com/johngrib/study-objects/pull/7 )

![image](https://user-images.githubusercontent.com/1855714/78038199-8406eb00-73a7-11ea-8bcb-f6194f96988b.png)

* [테스트 코드]( https://github.com/johngrib/study-objects/blob/cf3c2dd82ec37f91f5a4f46b0403df69d07555bb/src/test/java/com/johngrib/objects/_10_call/PhoneTest.java )

![image](https://user-images.githubusercontent.com/1855714/78038094-60dc3b80-73a7-11ea-9d2d-392d2213dd23.png)

## 챕터 08. 컬렉션

* [학습 코드]( https://github.com/johngrib/study-objects/pull/8 )

![image](https://user-images.githubusercontent.com/1855714/79059537-22535480-7cb6-11ea-97ee-64f5780a5bb5.png )

## 챕터 09. 에러 처리

* [학습 코드]( https://github.com/johngrib/study-objects/pull/9 )

![image](https://user-images.githubusercontent.com/1855714/79338814-dda01580-7f62-11ea-8606-f2ccb92dcae2.png)

* [테스트 코드]( https://github.com/johngrib/study-objects/pull/9/files#diff-24edaf9135c8c6658af8679293822043 )

![image](https://user-images.githubusercontent.com/1855714/79339072-412a4300-7f63-11ea-9e35-939f80ebbd06.png)

## 챕터 10. 제너릭 타입, 트레이트, 라이프타임

* [학습 코드](https://github.com/johngrib/study-objects/pull/10 )

![image](https://user-images.githubusercontent.com/1855714/79682471-88fbe380-825d-11ea-830f-054304abad8a.png) -->

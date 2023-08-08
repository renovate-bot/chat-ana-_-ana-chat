import Head from 'next/head'
import Image from 'next/image'
import { Inter } from '@next/font/google'
import styles from '@/styles/Home.module.css'
import { FormEvent, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/router'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  const [dom_v, dom_s] = useState<boolean>(false);
  const [members_v, members_s] = useState<boolean>(false);
  const {id} =  useRouter().query;
  console.log(id)
  let message = useRef<HTMLInputElement>(null);
  useEffect( () => {
    document.addEventListener("mousemove", (e) => {
      dom_s(e.clientX <= 15)
      members_s(window.innerWidth - e.clientX <= 10)
    })
  })
  const sendMessage = (e?: FormEvent<HTMLFormElement>) => {
    e?.preventDefault();

    if (message.current?.value){
      // let header: HeadersInit = new MainHeader();
      // let user: HeadersInit = new Headers();
      // header.set('chatid', 'a');
      // header.set('chat', user);
      fetch("http://127.0.0.1:8080/chat/info", {
        method: "POST",
        headers: {
          "chatid": "a",
          "chat": {
            "_id": "0",
            "sender": "a",
            "content": "asdf",
            "date": new Date().getTime()
          }
        }
      })
      message.current.value = "";
    }
  }
  return (
    <div id="main">
      <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@48,400,0,0" />

      <nav className={String(dom_v)}>
        <ServerBtn id="logo" select={true}/>
        
        <hr/>
        <ServerBtn id="1"/>
        <ServerBtn id="2"/>
        <ServerBtn id="3"/>
        <ServerBtn id="4"/>
        <ServerBtn id="5"/>
      </nav>

      <main>
        <section>메세지4</section>
        <section>메세지3</section>
        <section>메세지2</section>
        <section>메세지1</section>
      </main>
        
      <form id='messageSender' onSubmit={e => { sendMessage(e) }}>
          <input type="text" ref={message} placeholder='메세지를 입력해주세요' />
          <img src="/icon/send_w.svg" onClick={() => sendMessage()}/>
      </form>

      <aside className={String(members_v)}>
        <UserInfo id='1' name="eaaaaaaaaaaaaaaaa" url="/server/ea.png"/>
        <UserInfo id='1' name="eee" url="ea"/>
        <UserInfo id='1' name="s" url="ea"/>
        <UserInfo id='1' name="ddf" url="ea"/>
        <UserInfo id='1' name="5-23" url="ea"/>
      </aside>
    </div>
  )
}

function ServerBtn(props: {id: string, select?: boolean}){
  if (props.id == "logo") {
    return (
      <Link href={`?id=${props.id}`} className={`serverBtn ${props.select}`}>
        <img src={`/logo.svg`}/>
      </Link>
    )
  }
  return (
    <Link href={`?id=${props.id}`} className={`serverBtn ${props.select}`}>
      <img src={`/server/${props.id}.png`}/>
    </Link>
  )
}

function UserInfo(props: {url: string, id: string, name: string}){
  return (
    <a className={`userInfo`}>
      <img src={`/user/${props.id}.png`}/> <b>{props.name}</b>
      <span></span>
    </a>
  )
}
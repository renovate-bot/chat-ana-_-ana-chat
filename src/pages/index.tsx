import Head from 'next/head'
import Image from 'next/image'
import { Inter } from '@next/font/google'
import styles from '@/styles/Home.module.css'
import { FormEvent, useEffect, useRef, useState } from 'react'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  const [dom_v, dom_s] = useState<boolean>(false);
  const [members_v, members_s] = useState<boolean>(false);
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
      console.log(message.current?.value)
      message.current.value = "";
    }
  }
  return (
    <div id="main">
      <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@48,400,0,0" />

      <nav className={String(dom_v)}>
        <ServerBtn url="/logo.svg" select={true}/>
        
        <hr/>
        <ServerBtn url="/server/ea.png"/>
        <ServerBtn url="/server/ea.png"/>
        <ServerBtn url="/server/ea.png"/>
        <ServerBtn url="/server/ea.png"/>
        <ServerBtn url="/server/ea.png"/>
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
        <UserInfo id='eaaaaaaaaaaaaaaaaaaa1' url="/server/ea.png"/>
        <UserInfo id='eaaa2' url="/server/ea.png"/>
        <UserInfo id='eaaa3' url="/server/ea.png"/>
        <UserInfo id='eaaa4' url="/server/ea.png"/>
        <UserInfo id='eaaa5' url="/server/ea.png"/>
      </aside>
    </div>
  )
}

function ServerBtn(props: {url: string, select?: boolean}){
  // console.log(props.select)
  return (
    <a className={`serverBtn ${props.select}`}>
      <img src={props.url}/>
    </a>
  )
}

function UserInfo(props: {url: string, id: string}){
  // console.log(props.select)
  return (
    <a className={`userInfo`}>
      <img src={props.url}/> <b>{props.id}</b>
      <span></span>
    </a>
  )
}
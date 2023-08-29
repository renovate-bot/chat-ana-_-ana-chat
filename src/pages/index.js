"use client"
import Head from 'next/head'
import Image from 'next/image'
import { Inter } from 'next/font/google'
import { FormEvent, useEffect, useRef, useState } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/router'
import { useSearchParams } from 'next/navigation'

let cnt = 0;
let before = 0
console.log("started", before)

const inter = Inter({ subsets: ['latin'] })
export default function Home() {
  // open = false
  const [message_v, message_s] = useState(null);
  const [dom_v, dom_s] = useState(false);
  const [id_v, id_s] = useState(null);
  const [userName_v, userName_s] = useState(null);
  const [members_v, members_s] = useState(false);
  const {id, un} =  useRouter().query;
  
  const [msg_v, msg_s] = useState([])
  let message = useRef(null);
  // const searchParams = useSearchParams()
  // const idd = searchParams.get('id')
  
  useEffect( () => {
    members_s("")
    // let urlParams = new URLSearchParams(window.location.search);
    // _id = urlParams.get('id');
    id_s(id)
    userName_s(un)
    
    document.addEventListener("mousemove", (e) => {
      dom_s(e.clientX <= 15)
      members_s(window.innerWidth - e.clientX <= 10)
    })
    let f = async () => {
      if (cnt % 15 == 0 || cnt < 10){
        messageReload(null, message_s, window)
      }
      requestAnimationFrame(f)
      cnt++
    }
    // console.log("cnt: ", cnt)
    // console.log("idd: ", idd)
    // console.log("cnt == 0 & idd: ", cnt == 0 & idd != 0)
    if (cnt == 0) {
      fetch("http://127.0.0.1:8000/user/info", {
        method: "GET",
        headers: {
          "name": "a",
        }
      }).then(e => e.json().then(e => {
        for (let sv in e.servers) {
          console.log("server id", decodeURI(sv))
          document.getElementById("servers").innerHTML += `<a href="?id=${sv}&un=${e.name}" class="serverBtn"> <img src=/server/server.svg"/> </a>`
        }
        console.log("info", e)
      }))
      cnt++

      console.log("ok")
      f()
    }
  // if (!open){
  //   open = true
  // }

    
    }, [])
      
  const sendMessage = (e) => {
    e?.preventDefault();


    if (message.current?.value){
      fetch("http://127.0.0.1:8000/chat/send", {
        method: "POST",
        headers: {
          "sender": userName,
          "content": encodeURI(message.current.value),
          "servername": id
        }
      }).then(e => e.json().then(async msg => {
        
        messageReload(null, message_s, window)
        
      }))

      // fetch("http://127.0.0.1:8000/user/create", {
      //   method: "POST",
      //   headers: {
      //     "name": "524",
      //     "email": "yhanbyeol6bg@gmail.com",
      //     "profile_image": "a",
      //   }
      // }).then(e => { console.log(e) })




      // fetch("http://127.0.0.1:8000/server/join", {
      //   method: "POST",
      //   headers: {
      //     "servername": "a",
      //     "username": "524",
      //   }
      // }).then(e => { console.log(e) })

      // fetch("http://127.0.0.1:8000/server/create", {
      //   method: "POST",
      //   headers: {
      //     "name": "a"
      //   }
      // }).then( e => { console.log(e) })

      message.current.value = "";
    } else { console.log("non") } 
  }
  return (
    <div id="main">
      <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@48,400,0,0" />

      <nav className={String(dom_v)}>
        <ServerBtn id="logo" select={true}/>
        
        <hr/>
        <div id="servers">
          
          {/* <ServerBtn id="a"/>
          <ServerBtn id="b"/>
          <ServerBtn id="c"/>
          <ServerBtn id="d"/>
          <ServerBtn id="e"/> */}
        </div>
        {userName_v}
        {id_v}
      </nav>

      <main dangerouslySetInnerHTML={{__html: message_v}}></main>
        
      <form id='messageSender' onSubmit={e => { sendMessage(e) }}>
          <input type="text" ref={message} placeholder='메세지를 입력해주세요' />
          <img src="/icon/send_w.svg" onClick={() => sendMessage()}/>
      </form>

      <aside className={String(members_v)}>
        <UserInfo id='1' name="eaaaaaaaaaaaaaaaa" url="/server/ea.png"/>
        <UserInfo id='2' name="eee" url="ea"/>
        <UserInfo id='3' name="s" url="ea"/>
        <UserInfo id='4' name="ddf" url="ea"/>
        <UserInfo id='5' name="5-23" url="ea"/>
      </aside>
    </div>
  )
}
/**
 * 
 * @param {{
 *  id: string 
 *  select: boolean 
 * }} props 
 */
function ServerBtn(props){
  const {id, un} =  useRouter().query;
  if (props.id == "logo") {
    return (
      <Link href={`?id=${props.id}&un=${un}`} className={`serverBtn ${props.select}`}>
        <img src={`/logo.svg`}/>
      </Link>
    )
  }
  return (
    <Link href={`?id=${props.id}&un=${un}`} className={`serverBtn ${props.select}`}>
      <img src={`/server/server.svg`}/>
    </Link>
  )
}
/**
 * 
 * @param {{
 *  id: string,
 *  name: string,
 *  url: string
 * }} props 
 */
function UserInfo(props){
  return (
    <a className={`userInfo`}>
      <img src={`/user/1.png`}/> <b>{decodeURI(props.name)}</b>
      <span></span>
    </a>
  )
}

// function chatid(id) {
//   var a = {
//   content: null,
//   date: null,
//   sender: null
//   }
//   fetch("http://127.0.0.1:8000/chat/info", {
//     method: "GET",
//     headers: {
//       "chatid": id
//     }
//   }).then(e => e.json().then(e => {
//     a.content = e.content
//     a.date = e.date
//     a.sender = e.sender
//   }))
//   return a
// }

async function chatid(id) {
  const a = await fetch("http://127.0.0.1:8000/chat/info", {
    method: "GET",
    headers: {
      "chatid": id
    }
  })
  return await a.json()
}

/**
 * 
 * @param {string} id 
 * @param {import('react').Dispatch<import('react').SetStateAction<string>>} messages 
 * @param {Window} window 
 */
function messageReload(id, messages, window) {
  if (!id){
    let regex = (/id=(.*)&un=(.*)/gim).exec(window.location.search);
    id = regex[1]
  }
  fetch("http://127.0.0.1:8000/server/info", {
    headers: {
      "name": id
    }
  }).then(e => { e.json().then(e => {
    messages(e.html);
  }).catch(x => {
    messages("<h1>ERROR</h1>");
  })}).catch(x => {
    messages("<h1>ERROR</h1>");
  })
}
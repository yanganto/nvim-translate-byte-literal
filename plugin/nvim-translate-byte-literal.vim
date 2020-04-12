if !exists('s:translateByteLiteralJobId')
	let s:translateByteLiteralJobId = 0
endif

" Constants for RPC messages.
let s:TranslateByteArray = 'TranslateByteArray'

let s:bin = '~/.config/nvim/plugged/nvim-translate-byte-literal/target/debug'

" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "nvim-translate-byte-literal: cannot start rpc process"
  elseif -1 == id
    echoerr "nvim-translate-byte-literal: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:translateByteLiteralJobId = id 
    
    call s:configureCommands()
  endif
endfunction

function! s:configureCommands()
  command! TranslateByteArray :call s:translateByteArray()
endfunction

function! s:translateByteArray()
  call rpcnotify(s:translateByteLiteralJobId, s:TranslateByteArray, getpos('.'))
endfunction

function! s:initRpc()
  if s:translateByteLiteralJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return jobid
  else
    return s:translateByteLiteralJobId 
  endif
endfunction

call s:connect()

if !exists('s:translateByteLiteralJobId')
  let s:translateByteLiteralJobId = 0
endif

" Constants for RPC messages.
let s:TranslateByteArray = 'TranslateByteArray'

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
  if mode()=="v"
    let [line_start, column_start] = getpos("v")[1:2]
    let [line_end, column_end] = getpos(".")[1:2]
  else
    let [line_start, column_start] = getpos("'<")[1:2]
    let [line_end, column_end] = getpos("'>")[1:2]
  end

  if (line2byte(line_start)+column_start) > (line2byte(line_end)+column_end)
    let [line_start, column_start, line_end, column_end] =
    \ [line_end, column_end, line_start, column_start]
  end
  if line_start == line_end
    call rpcnotify(s:translateByteLiteralJobId, s:TranslateByteArray, getline("."))
  else
    call rpcnotify(s:translateByteLiteralJobId, s:TranslateByteArray, join(getline(line_start, line_end), '\n'))
  end
endfunction

function! s:initRpc()
  if s:translateByteLiteralJobId == 0
    let jobid = jobstart(['nix', 'run', 'github:yanganto/nvim-translate-byte-literal'], { 'rpc': v:true })
    return jobid
  else
    return s:translateByteLiteralJobId 
  endif
endfunction

call s:connect()

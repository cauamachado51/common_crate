$data = Get-Date -Format "dd/MM/yyyy HH:mm:ss"
Write-Output "pub fn data() -> &'static str { ""$data"" }"
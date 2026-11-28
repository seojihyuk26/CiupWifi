param(
    [string]$OutDir = (Join-Path (Split-Path -Parent $PSScriptRoot) "play-store\assets")
)

$ErrorActionPreference = "Stop"
Add-Type -AssemblyName System.Drawing

if (-not (Test-Path $OutDir)) {
    New-Item -ItemType Directory -Path $OutDir -Force | Out-Null
}

function New-RoundedRectPath {
    param([float]$X, [float]$Y, [float]$W, [float]$H, [float]$R)
    $path = New-Object System.Drawing.Drawing2D.GraphicsPath
    $d = [Math]::Min($R * 2, [Math]::Min($W, $H))
    if ($d -le 0) {
        $path.AddRectangle((New-Object System.Drawing.RectangleF($X, $Y, $W, $H)))
        return $path
    }
    $path.AddArc($X, $Y, $d, $d, 180, 90)
    $path.AddArc($X + $W - $d, $Y, $d, $d, 270, 90)
    $path.AddArc($X + $W - $d, $Y + $H - $d, $d, $d, 0, 90)
    $path.AddArc($X, $Y + $H - $d, $d, $d, 90, 90)
    $path.CloseFigure()
    return $path
}

function Draw-FilledRoundedRect {
    param(
        $G,
        [float]$X, [float]$Y, [float]$W, [float]$H, [float]$R,
        $FillBrush,
        $BorderPen = $null
    )
    $p = New-RoundedRectPath -X $X -Y $Y -W $W -H $H -R $R
    if ($null -ne $FillBrush) { $G.FillPath($FillBrush, $p) }
    if ($null -ne $BorderPen) { $G.DrawPath($BorderPen, $p) }
    $p.Dispose()
}

function Draw-WifiMark {
    param($G, [float]$CenterX, [float]$CenterY, [float]$Size)
    $scale = $Size / 100.0
    $dotY = $CenterY + 24 * $scale

    $glowRect = New-Object System.Drawing.RectangleF(($CenterX - 18 * $scale), ($dotY - 18 * $scale), (36 * $scale), (36 * $scale))
    $glowBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(70, 16, 185, 129))
    $G.FillEllipse($glowBrush, $glowRect)
    $glowBrush.Dispose()

    $dotRect = New-Object System.Drawing.RectangleF(($CenterX - 8 * $scale), ($dotY - 8 * $scale), (16 * $scale), (16 * $scale))
    $dotBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 52, 211, 153))
    $G.FillEllipse($dotBrush, $dotRect)
    $dotBrush.Dispose()

    $arcSpecs = @(
        @{ Radius = 24; Color = [System.Drawing.Color]::FromArgb(255, 165, 180, 252); Width = 6.5 },
        @{ Radius = 41; Color = [System.Drawing.Color]::FromArgb(255, 129, 140, 248); Width = 6.5 },
        @{ Radius = 58; Color = [System.Drawing.Color]::FromArgb(220, 99, 102, 241); Width = 6.5 }
    )
    foreach ($spec in $arcSpecs) {
        $rad = $spec.Radius * $scale
        $pen = New-Object System.Drawing.Pen($spec.Color, ($spec.Width * $scale))
        $pen.StartCap = [System.Drawing.Drawing2D.LineCap]::Round
        $pen.EndCap = [System.Drawing.Drawing2D.LineCap]::Round
        $rect = New-Object System.Drawing.RectangleF(($CenterX - $rad), ($dotY - $rad), ($rad * 2), ($rad * 2))
        $G.DrawArc($pen, $rect, 224, 92)
        $pen.Dispose()
    }
}

function Draw-Backdrop {
    param($G, [int]$W, [int]$H)
    $bgRect = New-Object System.Drawing.Rectangle(0, 0, $W, $H)
    $bgBrush = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        $bgRect,
        [System.Drawing.Color]::FromArgb(255, 9, 13, 22),
        [System.Drawing.Color]::FromArgb(255, 15, 36, 78),
        315.0
    )
    $G.FillRectangle($bgBrush, $bgRect)
    $bgBrush.Dispose()

    # Soft radial ambient glow at top
    $gp = New-Object System.Drawing.Drawing2D.GraphicsPath
    $gp.AddEllipse([float](-$W * 0.15), [float](-$H * 0.18), [float]($W * 1.3), [float]($H * 0.65))
    $pgb = New-Object System.Drawing.Drawing2D.PathGradientBrush($gp)
    $pgb.CenterColor = [System.Drawing.Color]::FromArgb(65, 99, 102, 241)
    $pgb.SurroundColors = @([System.Drawing.Color]::FromArgb(0, 99, 102, 241))
    $G.FillPath($pgb, $gp)
    $pgb.Dispose()
    $gp.Dispose()
}

function Draw-StringCentered {
    param($G, [string]$Text, $Font, $Brush, [float]$Y, [float]$CanvasWidth)
    $sf = New-Object System.Drawing.StringFormat
    $sf.Alignment = [System.Drawing.StringAlignment]::Center
    $sf.LineAlignment = [System.Drawing.StringAlignment]::Near
    $rect = New-Object System.Drawing.RectangleF(0, $Y, $CanvasWidth, 200)
    $G.DrawString($Text, $Font, $Brush, $rect, $sf)
    $sf.Dispose()
}

# 1) 512x512 Store Icon
function Build-Icon512 {
    $W = 512; $H = 512
    $bmp = New-Object System.Drawing.Bitmap($W, $H, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

    $rect = New-Object System.Drawing.Rectangle(0, 0, $W, $H)
    $bg = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        $rect,
        [System.Drawing.Color]::FromArgb(255, 15, 23, 42),
        [System.Drawing.Color]::FromArgb(255, 15, 43, 92),
        315.0
    )
    $g.FillRectangle($bg, $rect)
    $bg.Dispose()

    $gp = New-Object System.Drawing.Drawing2D.GraphicsPath
    $gp.AddEllipse(48, 48, 416, 416)
    $pgb = New-Object System.Drawing.Drawing2D.PathGradientBrush($gp)
    $pgb.CenterColor = [System.Drawing.Color]::FromArgb(75, 99, 102, 241)
    $pgb.SurroundColors = @([System.Drawing.Color]::FromArgb(0, 99, 102, 241))
    $g.FillPath($pgb, $gp)
    $pgb.Dispose()
    $gp.Dispose()

    Draw-WifiMark -G $g -CenterX 256 -CenterY 252 -Size 290

    $bmp.Save((Join-Path $OutDir "icon-512.png"), [System.Drawing.Imaging.ImageFormat]::Png)
    $g.Dispose()
    $bmp.Dispose()
}

# 2) 1024x500 Feature Graphic
function Build-FeatureGraphic {
    $W = 1024; $H = 500
    $bmp = New-Object System.Drawing.Bitmap($W, $H, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

    Draw-Backdrop -G $g -W $W -H $H

    # Glass badge on left
    $cardBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(190, 17, 24, 39))
    $cardPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(80, 129, 140, 248), 2.0)
    Draw-FilledRoundedRect -G $g -X 86 -Y 125 -W 250 -H 250 -R 52 -FillBrush $cardBrush -BorderPen $cardPen
    Draw-WifiMark -G $g -CenterX 211 -CenterY 248 -Size 175
    $cardBrush.Dispose()
    $cardPen.Dispose()

    # Typography on right
    $fTitle = New-Object System.Drawing.Font("Segoe UI", 56, [System.Drawing.FontStyle]::Bold)
    $fSub = New-Object System.Drawing.Font("Segoe UI", 22, [System.Drawing.FontStyle]::Bold)
    $fBody = New-Object System.Drawing.Font("Segoe UI", 16, [System.Drawing.FontStyle]::Regular)
    $fPill = New-Object System.Drawing.Font("Segoe UI", 13, [System.Drawing.FontStyle]::Bold)

    $bWhite = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 248, 250, 252))
    $bEmerald = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 52, 211, 153))
    $bMuted = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 203, 213, 225))
    $bIndigoText = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 199, 210, 254))

    $g.DrawString("CiupWifi", $fTitle, $bWhite, 382, 118)
    $g.DrawString("Auto-Login for CIUP WifiCity", $fSub, $bEmerald, 386, 206)
    $g.DrawString("Stay connected on the CIUP WifiCity network.", $fBody, $bMuted, 386, 254)

    $pillFill = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(160, 30, 41, 59))
    $pillPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(90, 99, 102, 241), 1.5)
    $pills = @(
        @{ Text = "No Location"; X = 386; W = 172 },
        @{ Text = "Event Driven"; X = 572; W = 136 },
        @{ Text = "Open Source"; X = 722; W = 140 }
    )
    foreach ($pl in $pills) {
        Draw-FilledRoundedRect -G $g -X $pl.X -Y 316 -W $pl.W -H 40 -R 20 -FillBrush $pillFill -BorderPen $pillPen
        $sf = New-Object System.Drawing.StringFormat
        $sf.Alignment = [System.Drawing.StringAlignment]::Center
        $sf.LineAlignment = [System.Drawing.StringAlignment]::Center
        $g.DrawString($pl.Text, $fPill, $bIndigoText, (New-Object System.Drawing.RectangleF($pl.X, 316, $pl.W, 40)), $sf)
        $sf.Dispose()
    }

    $pillFill.Dispose(); $pillPen.Dispose()
    $fTitle.Dispose(); $fSub.Dispose(); $fBody.Dispose(); $fPill.Dispose()
    $bWhite.Dispose(); $bEmerald.Dispose(); $bMuted.Dispose(); $bIndigoText.Dispose()

    $bmp.Save((Join-Path $OutDir "feature-graphic-1024x500.png"), [System.Drawing.Imaging.ImageFormat]::Png)
    $g.Dispose(); $bmp.Dispose()
}

function Draw-PhoneShell {
    param($G, [string]$Headline, [string]$Subheadline)
    $W = 1080; $H = 1920
    Draw-Backdrop -G $G -W $W -H $H

    $fHead = New-Object System.Drawing.Font("Segoe UI", 42, [System.Drawing.FontStyle]::Bold)
    $fSub = New-Object System.Drawing.Font("Segoe UI", 22, [System.Drawing.FontStyle]::Regular)
    $bWhite = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 248, 250, 252))
    $bSub = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 148, 163, 184))

    Draw-StringCentered -G $G -Text $Headline -Font $fHead -Brush $bWhite -Y 108 -CanvasWidth $W
    Draw-StringCentered -G $G -Text $Subheadline -Font $fSub -Brush $bSub -Y 192 -CanvasWidth $W

    # Phone mockup frame
    $frameBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(235, 11, 16, 28))
    $framePen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(90, 148, 163, 184), 4.0)
    Draw-FilledRoundedRect -G $G -X 130 -Y 300 -W 820 -H 1500 -R 64 -FillBrush $frameBrush -BorderPen $framePen

    # Speaker / camera notch bar
    $notchBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 30, 41, 59))
    Draw-FilledRoundedRect -G $G -X 470 -Y 332 -W 140 -H 14 -R 7 -FillBrush $notchBrush -BorderPen $null

    $fHead.Dispose(); $fSub.Dispose(); $bWhite.Dispose(); $bSub.Dispose()
    $frameBrush.Dispose(); $framePen.Dispose(); $notchBrush.Dispose()
}

# 3) Screenshot 1 - Connected Status
function Build-Screenshot1 {
    $W = 1080; $H = 1920
    $bmp = New-Object System.Drawing.Bitmap($W, $H, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

    Draw-PhoneShell -G $g -Headline "Auto-Connect on WifiCity" -Subheadline "Sign in once - stays online across captive renewals"

    # Inner app card
    $cardBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(220, 17, 24, 39))
    $cardPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(40, 255, 255, 255), 2.0)
    Draw-FilledRoundedRect -G $g -X 200 -Y 460 -W 680 -H 1080 -R 42 -FillBrush $cardBrush -BorderPen $cardPen

    # Emerald glowing status circle
    $ringBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(40, 16, 185, 129))
    $ringPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(140, 16, 185, 129), 3.0)
    $g.FillEllipse($ringBrush, 488, 536, 104, 104)
    $g.DrawEllipse($ringPen, 488, 536, 104, 104)
    $dotBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 16, 185, 129))
    $g.FillEllipse($dotBrush, 520, 568, 40, 40)

    $fCardTitle = New-Object System.Drawing.Font("Segoe UI", 34, [System.Drawing.FontStyle]::Bold)
    $fCardSub = New-Object System.Drawing.Font("Segoe UI", 18, [System.Drawing.FontStyle]::Regular)
    $fLabel = New-Object System.Drawing.Font("Segoe UI", 17, [System.Drawing.FontStyle]::Regular)
    $fVal = New-Object System.Drawing.Font("Segoe UI", 17, [System.Drawing.FontStyle]::Bold)
    $fBtn = New-Object System.Drawing.Font("Segoe UI", 19, [System.Drawing.FontStyle]::Bold)

    $bWhite = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 248, 250, 252))
    $bMuted = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 148, 163, 184))
    $bEmerald = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 52, 211, 153))
    $bDanger = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 252, 165, 165))

    Draw-StringCentered -G $g -Text "Connected" -Font $fCardTitle -Brush $bWhite -Y 668 -CanvasWidth $W
    Draw-StringCentered -G $g -Text "Cite Universitaire  •  WifiCity" -Font $fCardSub -Brush $bMuted -Y 738 -CanvasWidth $W

    # Info group box
    $infoBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(180, 15, 23, 42))
    $infoPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(30, 255, 255, 255), 1.5)
    Draw-FilledRoundedRect -G $g -X 256 -Y 810 -W 568 -H 320 -R 24 -FillBrush $infoBrush -BorderPen $infoPen

    $rows = @(
        @{ L = "Network"; V = "Online (Auto)"; C = $bEmerald; Y = 838 },
        @{ L = "Wi-Fi"; V = "WifiCity"; C = $bWhite; Y = 908 },
        @{ L = "Account"; V = "resident_ciup"; C = $bWhite; Y = 978 },
        @{ L = "Trigger"; V = "Event-Driven"; C = $bEmerald; Y = 1048 }
    )
    $sfRight = New-Object System.Drawing.StringFormat
    $sfRight.Alignment = [System.Drawing.StringAlignment]::Far
    foreach ($r in $rows) {
        $g.DrawString($r.L, $fLabel, $bMuted, 290, $r.Y)
        $g.DrawString($r.V, $fVal, $r.C, (New-Object System.Drawing.RectangleF(290, $r.Y, 500, 44)), $sfRight)
    }

    # Action buttons
    $priBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 79, 70, 229))
    Draw-FilledRoundedRect -G $g -X 256 -Y 1190 -W 568 -H 82 -R 22 -FillBrush $priBrush -BorderPen $null
    $sfCenter = New-Object System.Drawing.StringFormat
    $sfCenter.Alignment = [System.Drawing.StringAlignment]::Center
    $sfCenter.LineAlignment = [System.Drawing.StringAlignment]::Center
    $g.DrawString("Check Connection", $fBtn, $bWhite, (New-Object System.Drawing.RectangleF(256, 1190, 568, 82)), $sfCenter)

    $discBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(40, 239, 68, 68))
    $discPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(90, 239, 68, 68), 1.5)
    Draw-FilledRoundedRect -G $g -X 256 -Y 1296 -W 568 -H 74 -R 20 -FillBrush $discBrush -BorderPen $discPen
    $g.DrawString("Disconnect WifiCity", $fBtn, $bDanger, (New-Object System.Drawing.RectangleF(256, 1296, 568, 74)), $sfCenter)

    $ghostBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(140, 30, 41, 59))
    $ghostPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(40, 255, 255, 255), 1.5)
    Draw-FilledRoundedRect -G $g -X 256 -Y 1392 -W 568 -H 74 -R 20 -FillBrush $ghostBrush -BorderPen $ghostPen
    $g.DrawString("Change Account", $fBtn, $bMuted, (New-Object System.Drawing.RectangleF(256, 1392, 568, 74)), $sfCenter)

    $sfRight.Dispose(); $sfCenter.Dispose()
    $cardBrush.Dispose(); $cardPen.Dispose(); $ringBrush.Dispose(); $ringPen.Dispose(); $dotBrush.Dispose()
    $infoBrush.Dispose(); $infoPen.Dispose(); $priBrush.Dispose(); $discBrush.Dispose(); $discPen.Dispose()
    $ghostBrush.Dispose(); $ghostPen.Dispose()
    $fCardTitle.Dispose(); $fCardSub.Dispose(); $fLabel.Dispose(); $fVal.Dispose(); $fBtn.Dispose()
    $bWhite.Dispose(); $bMuted.Dispose(); $bEmerald.Dispose(); $bDanger.Dispose()

    $bmp.Save((Join-Path $OutDir "screenshot-1-connected.png"), [System.Drawing.Imaging.ImageFormat]::Png)
    $g.Dispose(); $bmp.Dispose()
}

# 4) Screenshot 2 - Login Setup
function Build-Screenshot2 {
    $W = 1080; $H = 1920
    $bmp = New-Object System.Drawing.Bitmap($W, $H, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

    Draw-PhoneShell -G $g -Headline "One-Time 10-Second Setup" -Subheadline "Credentials stay in private on-device app storage"

    $cardBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(220, 17, 24, 39))
    $cardPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(40, 255, 255, 255), 2.0)
    Draw-FilledRoundedRect -G $g -X 200 -Y 460 -W 680 -H 1000 -R 42 -FillBrush $cardBrush -BorderPen $cardPen

    $badgeBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(55, 99, 102, 241))
    $badgePen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(110, 129, 140, 248), 2.0)
    Draw-FilledRoundedRect -G $g -X 474 -Y 530 -W 132 -H 132 -R 34 -FillBrush $badgeBrush -BorderPen $badgePen
    Draw-WifiMark -G $g -CenterX 540 -CenterY 596 -Size 88

    $fCardTitle = New-Object System.Drawing.Font("Segoe UI", 34, [System.Drawing.FontStyle]::Bold)
    $fCardSub = New-Object System.Drawing.Font("Segoe UI", 18, [System.Drawing.FontStyle]::Regular)
    $fLabel = New-Object System.Drawing.Font("Segoe UI", 16, [System.Drawing.FontStyle]::Bold)
    $fInput = New-Object System.Drawing.Font("Segoe UI", 19, [System.Drawing.FontStyle]::Regular)
    $fBtn = New-Object System.Drawing.Font("Segoe UI", 20, [System.Drawing.FontStyle]::Bold)

    $bWhite = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 248, 250, 252))
    $bMuted = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 148, 163, 184))

    Draw-StringCentered -G $g -Text "CiupWifi" -Font $fCardTitle -Brush $bWhite -Y 692 -CanvasWidth $W
    Draw-StringCentered -G $g -Text "Auto-login for CIUP Campus Network" -Font $fCardSub -Brush $bMuted -Y 762 -CanvasWidth $W

    $inBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(210, 15, 23, 42))
    $inPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(80, 148, 163, 184), 2.0)

    $g.DrawString("Username", $fLabel, $bMuted, 260, 848)
    Draw-FilledRoundedRect -G $g -X 256 -Y 892 -W 568 -H 82 -R 20 -FillBrush $inBrush -BorderPen $inPen
    $g.DrawString("resident_ciup", $fInput, $bWhite, 290, 916)

    $g.DrawString("Password", $fLabel, $bMuted, 260, 1006)
    Draw-FilledRoundedRect -G $g -X 256 -Y 1050 -W 568 -H 82 -R 20 -FillBrush $inBrush -BorderPen $inPen
    $g.DrawString("••••••••••••", $fInput, $bWhite, 290, 1072)

    $priBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 79, 70, 229))
    Draw-FilledRoundedRect -G $g -X 256 -Y 1196 -W 568 -H 88 -R 22 -FillBrush $priBrush -BorderPen $null
    $sfCenter = New-Object System.Drawing.StringFormat
    $sfCenter.Alignment = [System.Drawing.StringAlignment]::Center
    $sfCenter.LineAlignment = [System.Drawing.StringAlignment]::Center
    $g.DrawString("Save & Connect", $fBtn, $bWhite, (New-Object System.Drawing.RectangleF(256, 1196, 568, 88)), $sfCenter)

    Draw-StringCentered -G $g -Text "WifiCity Auto-Portal (All Ports)" -Font $fCardSub -Brush $bMuted -Y 1344 -CanvasWidth $W

    $sfCenter.Dispose(); $cardBrush.Dispose(); $cardPen.Dispose()
    $badgeBrush.Dispose(); $badgePen.Dispose(); $inBrush.Dispose(); $inPen.Dispose(); $priBrush.Dispose()
    $fCardTitle.Dispose(); $fCardSub.Dispose(); $fLabel.Dispose(); $fInput.Dispose(); $fBtn.Dispose()
    $bWhite.Dispose(); $bMuted.Dispose()

    $bmp.Save((Join-Path $OutDir "screenshot-2-login.png"), [System.Drawing.Imaging.ImageFormat]::Png)
    $g.Dispose(); $bmp.Dispose()
}

# 5) Screenshot 3 - Privacy & Battery Highlights
function Build-Screenshot3 {
    $W = 1080; $H = 1920
    $bmp = New-Object System.Drawing.Bitmap($W, $H, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

    Draw-Backdrop -G $g -W $W -H $H

    $fHead = New-Object System.Drawing.Font("Segoe UI", 42, [System.Drawing.FontStyle]::Bold)
    $fSub = New-Object System.Drawing.Font("Segoe UI", 22, [System.Drawing.FontStyle]::Regular)
    $fCardH = New-Object System.Drawing.Font("Segoe UI", 26, [System.Drawing.FontStyle]::Bold)
    $fCardP = New-Object System.Drawing.Font("Segoe UI", 19, [System.Drawing.FontStyle]::Regular)

    $bWhite = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 248, 250, 252))
    $bSub = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 148, 163, 184))
    $bLight = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 203, 213, 225))
    $bEmerald = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 52, 211, 153))
    $bIndigo = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 165, 180, 252))

    Draw-StringCentered -G $g -Text "No Tracking. Low Power." -Font $fHead -Brush $bWhite -Y 130 -CanvasWidth $W
    Draw-StringCentered -G $g -Text "Built with strict privacy and OS efficiency" -Font $fSub -Brush $bSub -Y 214 -CanvasWidth $W

    $cardBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(205, 17, 24, 39))
    $cardPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(65, 129, 140, 248), 2.0)

    $cards = @(
        @{
            Title = "Event-Driven & Low Power"
            Body = "Uses Android's native ConnectivityManager network triggers instead of background polling loops or persistent foreground notifications."
            Tag = "EVENT-DRIVEN"
            TagBrush = $bEmerald
            Y = 380
        },
        @{
            Title = "Zero Sensitive Permissions"
            Body = "Never asks for Location, Notifications, Contacts, or Camera access. Only basic network state permissions required to bind portal traffic."
            Tag = "PRIVACY FIRST"
            TagBrush = $bIndigo
            Y = 820
        },
        @{
            Title = "100% On-Device Storage"
            Body = "Your campus login never leaves your device except to talk directly to your local CIUP portal gateway. Fully open source (MIT)."
            Tag = "OPEN SOURCE"
            TagBrush = $bEmerald
            Y = 1260
        }
    )

    foreach ($c in $cards) {
        Draw-FilledRoundedRect -G $g -X 110 -Y $c.Y -W 860 -H 370 -R 36 -FillBrush $cardBrush -BorderPen $cardPen
        $g.DrawString($c.Tag, $fCardP, $c.TagBrush, 168, ($c.Y + 48))
        $g.DrawString($c.Title, $fCardH, $bWhite, 168, ($c.Y + 96))
        $bodyRect = New-Object System.Drawing.RectangleF(168, ($c.Y + 168), 744, 170)
        $g.DrawString($c.Body, $fCardP, $bLight, $bodyRect)
    }

    $cardBrush.Dispose(); $cardPen.Dispose()
    $fHead.Dispose(); $fSub.Dispose(); $fCardH.Dispose(); $fCardP.Dispose()
    $bWhite.Dispose(); $bSub.Dispose(); $bLight.Dispose(); $bEmerald.Dispose(); $bIndigo.Dispose()

    $bmp.Save((Join-Path $OutDir "screenshot-3-privacy.png"), [System.Drawing.Imaging.ImageFormat]::Png)
    $g.Dispose(); $bmp.Dispose()
}

Build-Icon512
Build-FeatureGraphic
Build-Screenshot1
Build-Screenshot2
Build-Screenshot3
Write-Output "GENERATED_ALL_PLAY_ASSETS"
Get-ChildItem $OutDir | Select-Object Name, Length

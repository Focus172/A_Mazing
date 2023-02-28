package code

import Interfaces.ImageHelper.Companion.convertToBoolean
import Interfaces.ImageHelper.Companion.readImage
import java.awt.Color
import java.awt.Dimension
import java.awt.Graphics
import java.awt.Graphics2D
import java.awt.event.ActionEvent
import java.awt.event.ActionListener
import java.awt.image.BufferedImage
import javax.swing.JPanel

class Amazing : JPanel(), Runnable, ActionListener {
    //public static final InputManager inpMan = new InputManager();
    private var gameThread: Thread
    var mazeWall: Array<BooleanArray>
    var mazeImage: BufferedImage

    companion object {
        // { //, KeyListener {
        // controlling the frame rate
        const val FPS = 60

        // controls the size of the board
        const val TILE_SIZE = 8
        const val MAX_COL = 64
        const val MAX_ROW = 64
        const val MAX_X = MAX_COL * TILE_SIZE
        const val MAX_Y = MAX_ROW * TILE_SIZE

        // suppress serialization warning
        private const val serialVersionUID = 490905409104883233L
    }

    init {
        //--------------------setting up window-------------------
        this.preferredSize = Dimension(MAX_X, MAX_Y)
        background = Color.white
        this.isDoubleBuffered = true

        //this.addKeyListener(inpMan);
        this.isFocusable = true

        //-------------------reading image------------------------
        //setting parameters
        val imagePath = "./inp1.png"
        // val imagePath = ../../inp2.png
        //inp 1
        val width = 64
        val height = 64

        //inp 2
        //int width = 401;
        //int height = 401;
        mazeImage = readImage(imagePath, width, height).getOrElse {
            throw it
        }

        mazeWall = convertToBoolean(mazeImage)


        //prints maze - not necessary
        //printMaze(mazeWall);

        //--------------------finding path------------------------

        //declare start and end point
        var startNodeX = 0
        for (sq in mazeWall[0].indices)  //this only works if there is one open Objects.node, to up scale change this code and stop being dumb
            if (mazeWall[0][sq]) startNodeX = sq
        // starts Objects.node Y is implied to be 0 which should always be true unless it isn't but that would be bad so hopefully that doesn't happen
        var endNodeX = 0
        val endNodeY = height - 1
        for (sq in mazeWall[height - 1].indices) if (mazeWall[height - 1][sq]) endNodeX = sq

        //maze.solveMaze();

        // start game thread
        gameThread = Thread(this)
        gameThread.start()
    }

    override fun run() {
        var delta = 0.0
        var lastTime = System.nanoTime()
        var curTime: Long
        var diff: Long = 0
        val nanoFrame = (1000000000 / FPS).toDouble()
        while (gameThread != null) {
            curTime = System.nanoTime()
            diff = curTime - lastTime
            delta += diff / nanoFrame
            lastTime = curTime
            if (delta >= 1) {
                tickAll()
                delta--
            }
        }
    }

    override fun paintComponent(g: Graphics) {
        super.paintComponent(g)
        // when calling g.drawImage() we can use "this" for the ImageObserver
        // because Component implements the ImageObserver interface, and JPanel
        // extends from Component. So "this" Board instance, as a Component, can
        // react to imageUpdate() events triggered by g.drawImage()

        //g.fillRect(x, y, length, width);
        val g2 = g as Graphics2D
        g2.color = Color.black
        for (r in mazeWall.indices) {
            for (c in mazeWall[r].indices) {
                if (mazeWall[r][c]) {
                    g2.fillRect(r * TILE_SIZE, c * TILE_SIZE, 8, 8)
                }
            }
        }

        //p.draw(g2);

        //Point mousePos = MouseInfo.getPointerInfo().getLocation();
        //g2.fillRect(mousePos.x, mousePos.y, 10, 10);
        g2.dispose()


        /*
            //paint the level select

            //Point p = MouseInfo.getPointerInfo().getLocation()

            int buffer = 50;
            int textSize = 78;

            int thingSize = 50;



            JButton myButton = new JButton("My Button");
            add(myButton);

            for (int i = 1; i <= 4; i++) {
                g.setColor(new Color(30, 201, 139));
                g.fillRect(thingSize, i*thingSize, MAX_X-2*thingSize, thingSize);



                if (i == 1) {
                    g.setColor(new Color(255, 0, 0));
                    g.fillRect(0, i*thingSize, thingSize, thingSize);
                }
            }

            // this smooths out animations on some systems
            Toolkit.getDefaultToolkit().sync();

            // you are in the game loop

            //BufferedImage img;

            // draw our graphics.

            //drawing the health

            // set the text to be displayed
            //String text = "60" + "/100";

            // we need to cast the Graphics to Graphics2D to draw nicer text
            // set the text color and font

            //filling green section

            / *
            g.setColor(new Color(30, 201, 139));
            g.fillRect(0, MAX_Y, (int)((60/100.0)*MAX_X), HEALTH_BAR_Y);

            //filling red section
            g.setColor(new Color(200, 0, 0));
            g.fillRect((int)((60/100.0)*MAX_X), MAX_Y, MAX_X, HEALTH_BAR_Y);



            // this smooths out animations on some systems

        Toolkit.getDefaultToolkit().sync();
        */
    }

    fun tickAll() {
        //passing data to mazeSolver
        //Pathfinder maze = new Pathfinder(startNodeX, endNodeX, endNodeY, mazeWall);

        //------------------------write path--------------------------------
        //maze.getPath();
        //image.setRGB(0, 0, width, height, imagePixels, 0, width);
        run()
        repaint()
    }

    override fun actionPerformed(e: ActionEvent) {
        // this method is called by the timer every DELAY ms.
        // use this space to update the state of your game or animation
        // before the graphics are redrawn.

        /*
        if (onMenu) {
            //gaming
            //set variables
            tickAll(); // has special case for onMenu
        } else {
            tickAll();
        }
        */
    }
}
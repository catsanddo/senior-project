import pygame as pg
pg.init()


class Entity:
    def __init__(self, x, y, sprite):
        self.x = x
        self.y = y
        self.sprite = pg.image.load(sprite)

    def draw(self, surface):
        surface.blit(self.sprite, (self.x, self.y))


def main():
    SCREEN = (512, 448)
    screen = pg.display.set_mode(SCREEN)
    clock = pg.time.Clock()
    FPS = 30

    pg.key.set_repeat(250, 50)

    s_buffer = pg.Surface((256, 224))

    player = Entity(0, 0, 'art/player.png')

    running = True
    while running:
        for e in pg.event.get():
            if e.type == pg.QUIT:
                running = False
            if e.type == pg.KEYDOWN:
                if e.key == pg.K_UP:
                    player.y -= 3
                if e.key == pg.K_DOWN:
                    player.y += 3
                if e.key == pg.K_LEFT:
                    player.x -= 3
                if e.key == pg.K_RIGHT:
                    player.x += 3

        s_buffer.fill((0, 0, 0))
        player.draw(s_buffer)

        screen.blit(pg.transform.scale(s_buffer, SCREEN), (0, 0))
        pg.display.flip()

        clock.tick(FPS)


if __name__ == '__main__':
    main()
    pg.quit()
    quit()

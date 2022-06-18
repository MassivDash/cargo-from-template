
import { Request, Response, NextFunction } from 'express';

export const %name% =
  async (
    req: Request,
    res: Response,
    next: NextFunction,
  ): Promise<Response | undefined> => {
    try {
     

    } catch (e) {
      next(e);
    }
  };
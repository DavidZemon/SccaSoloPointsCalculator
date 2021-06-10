import { Component, PropsWithoutRef } from 'react';
import { Col, Row } from 'react-bootstrap';
import { ChampionshipResults as ChampionshipResultsData } from '../models';
import { PaxService } from '../services';

interface ChampionshipResultsProps extends PropsWithoutRef<any> {
  paxService: PaxService;
  results?: ChampionshipResultsData;
}

interface ChampionshipResultsState {}

export class ChampionshipResults extends Component<
  ChampionshipResultsProps,
  ChampionshipResultsState
> {
  constructor(props: Readonly<ChampionshipResultsProps>) {
    super(props);
    this.state = {};
  }

  render() {
    if (
      this.props.results &&
      Object.values(this.props.results).some((v) => !!v)
    ) {
      return (
        <Row>
          <Col>
            <h2>Championship Results</h2>
          </Col>
        </Row>
      );
    } else {
      return null;
    }
  }
}
